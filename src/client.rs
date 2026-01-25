//! Clean channel-based MQTT client implementation
//!
//! This implementation uses pure channel communication and abstracted underlying layer
//! to ensure complete separation and testability

#[cfg(target_arch = "wasm32")]
use crate::log;
#[cfg(target_arch = "wasm32")]
use crate::websocket::BrowserWebSocket;
use crate::{error::*, types::*, websocket::*};
use futures::channel::{mpsc, oneshot};
use futures::stream::StreamExt;
use futures::{select, FutureExt};
use mqtt_protocol_core::mqtt;
use mqtt_protocol_core::mqtt::prelude::*;
use std::collections::HashSet;

/// Requests from public API to internal processor
#[derive(Debug)]
pub enum Request {
    /// Connect WebSocket to broker
    Connect {
        url: String,
        reply: oneshot::Sender<Result<()>>,
    },
    /// Send a packet
    Send {
        packet: mqtt::packet::Packet,
        reply: oneshot::Sender<Result<()>>,
    },
    /// Receive a packet
    Recv {
        reply: oneshot::Sender<Result<mqtt::packet::Packet>>,
    },
    /// Close connection
    Close { reply: oneshot::Sender<Result<()>> },
    /// Get connection state
    State {
        reply: oneshot::Sender<ConnectionState>,
    },
    /// Check if connected
    IsConnected { reply: oneshot::Sender<bool> },
    /// Acquire packet ID
    AcquirePacketId { reply: oneshot::Sender<Option<u16>> },
    /// Register packet ID
    RegisterPacketId {
        packet_id: u16,
        reply: oneshot::Sender<bool>,
    },
    /// Release packet ID
    ReleasePacketId {
        packet_id: u16,
        reply: oneshot::Sender<Result<()>>,
    },
}

/// MQTT client with clean channel-based design
pub struct MqttClient {
    request_sender: mpsc::UnboundedSender<Request>,
}

/// Internal MQTT processor
struct MqttProcessor {
    config: MqttConfig,
    state: ConnectionState,
    mqtt_connection: mqtt::Connection<mqtt::role::Client>,

    // Buffer management
    read_buffer: Vec<u8>,
    buffer_size: usize,
    consumed_bytes: usize,

    // Timer management - tracks which timers are active
    // Actual timer handling is done by the underlying layer
    active_timers: HashSet<String>,

    // Packet handling
    packet_sender: mpsc::UnboundedSender<mqtt::packet::Packet>,
    packet_receiver: mpsc::UnboundedReceiver<mqtt::packet::Packet>,
    pending_recv_requests: Vec<oneshot::Sender<Result<mqtt::packet::Packet>>>,
    undelivered_packet: Option<mqtt::packet::Packet>,

    // WebSocket communication
    websocket_events: mpsc::UnboundedReceiver<UnderlyingLayerEvent>,
    websocket_commands: mpsc::UnboundedSender<UnderlyingLayerCommand>,

    // Request sender for timer callbacks
    #[allow(dead_code)]
    request_sender: mpsc::UnboundedSender<Request>,
}

impl MqttProcessor {
    fn new<W: UnderlyingLayerInterface>(
        config: MqttConfig,
        mut websocket: W,
        request_sender: mpsc::UnboundedSender<Request>,
    ) -> (Self, W) {
        let mqtt_connection = mqtt::Connection::<mqtt::role::Client>::new(config.version);
        let (packet_sender, packet_receiver) = mpsc::unbounded();
        let websocket_events = websocket.event_receiver();
        let websocket_commands = websocket.command_sender();

        let processor = Self {
            config,
            state: ConnectionState::Disconnected,
            mqtt_connection,
            read_buffer: Vec::with_capacity(8192),
            buffer_size: 0,
            consumed_bytes: 0,
            active_timers: HashSet::new(),
            packet_sender,
            packet_receiver,
            pending_recv_requests: Vec::new(),
            undelivered_packet: None,
            websocket_events,
            websocket_commands,
            request_sender,
        };

        (processor, websocket)
    }

    /// Main processing loop
    async fn run(&mut self, mut request_receiver: mpsc::UnboundedReceiver<Request>) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &"★★★ MQTT: Starting MQTT processor main loop (NEW VERSION) ★★★".into(),
        );

        // Configure MQTT connection
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &format!(
                "Config pingreq_send_interval_ms: {:?}",
                self.config.pingreq_send_interval_ms
            )
            .into(),
        );

        if let Some(interval) = self.config.pingreq_send_interval_ms {
            self.mqtt_connection
                .set_pingreq_send_interval(Some(interval));
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Set pingreq_send_interval to {}ms", interval).into());
        } else {
            // Use default interval based on keep_alive (will be set when CONNECT packet is processed)
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(
                &"pingreq_send_interval will be auto-configured from CONNECT keep_alive".into(),
            );
        }
        self.mqtt_connection
            .set_auto_pub_response(self.config.auto_pub_response);
        self.mqtt_connection
            .set_auto_ping_response(self.config.auto_ping_response);
        self.mqtt_connection
            .set_auto_map_topic_alias_send(self.config.auto_map_topic_alias_send);
        self.mqtt_connection
            .set_auto_replace_topic_alias_send(self.config.auto_replace_topic_alias_send);
        self.mqtt_connection
            .set_pingresp_recv_timeout(self.config.pingresp_recv_timeout_ms);

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"MQTT connection configured".into());

        loop {
            select! {
                // Handle API requests
                request = request_receiver.next().fuse() => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Received API request".into());
                    match request {
                        Some(req) => {
                            if !self.handle_request(req).await {
                                break; // Close request received
                            }
                        }
                        None => break, // Request channel closed
                    }
                }

                // Handle WebSocket events
                event = self.websocket_events.next().fuse() => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Received WebSocket event".into());
                    if let Some(event) = event {
                        // Handle the event - do NOT break on close to allow reconnection
                        self.handle_websocket_event(event).await;
                    } else {
                        // WebSocket event stream ended
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(&"WebSocket event stream ended".into());
                        break;
                    }
                }

                // Handle received packets and forward to pending recv requests
                packet = self.packet_receiver.next().fuse() => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Received packet from MQTT connection".into());
                    if let Some(packet) = packet {
                        self.handle_received_packet(packet);
                    }
                }

            }
        }
    }

    /// Handle API requests
    async fn handle_request(&mut self, request: Request) -> bool {
        match request {
            Request::Connect { url, reply } => {
                let _ = self.connect(&url, reply).await;
                // Reply is always handled inside connect() method
            }
            Request::Send { packet, reply } => {
                let result = self.send_packet(packet).await;
                let _ = reply.send(result);
            }
            Request::Recv { reply } => {
                // Check if there's an undelivered packet from previous recv() timeout
                if let Some(packet) = self.undelivered_packet.take() {
                    // Deliver the saved packet immediately
                    let _ = reply.send(Ok(packet));
                } else {
                    // Queue the recv request to be fulfilled when packet arrives
                    self.pending_recv_requests.push(reply);
                }
            }
            Request::Close { reply } => {
                let result = self.close().await;
                let _ = reply.send(result);
                // Do NOT exit the loop - allow reconnection by continuing to process requests
                // The loop only exits when the request channel is closed (client dropped)
            }
            Request::State { reply } => {
                let _ = reply.send(self.state);
            }
            Request::IsConnected { reply } => {
                let _ = reply.send(matches!(self.state, ConnectionState::Connected));
            }
            Request::AcquirePacketId { reply } => {
                let packet_id = self.mqtt_connection.acquire_packet_id().ok();
                let _ = reply.send(packet_id);
            }
            Request::RegisterPacketId { packet_id, reply } => {
                let result = self.mqtt_connection.register_packet_id(packet_id).is_ok();
                let _ = reply.send(result);
            }
            Request::ReleasePacketId { packet_id, reply } => {
                let events = self.mqtt_connection.release_packet_id(packet_id);
                let _ = self.handle_mqtt_events(events);
                let _ = reply.send(Ok(()));
            }
        }
        true
    }

    /// Handle WebSocket events
    async fn handle_websocket_event(&mut self, event: UnderlyingLayerEvent) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Handling WebSocket event: {:?}", event).into());

        match event {
            UnderlyingLayerEvent::Connected => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"WebSocket Connected event - updating state".into());
                self.state = ConnectionState::Connected;
            }
            UnderlyingLayerEvent::Message(data) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(
                    &format!(
                        "🔥 NEW VERSION: WebSocket Message event - processing {} bytes",
                        data.len()
                    )
                    .into(),
                );
                self.process_incoming_data(data);
            }
            UnderlyingLayerEvent::Error(_error) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("WebSocket Error event: {}", _error).into());
                self.state = ConnectionState::Disconnected;
            }
            UnderlyingLayerEvent::Closed => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"WebSocket Closed event - updating state".into());
                self.state = ConnectionState::Closed;
                let events = self.mqtt_connection.notify_closed();
                let _ = self.handle_mqtt_events(events);

                // Clear any remaining timers
                self.active_timers.clear();
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"All timers cleared on connection close".into());
            }
            UnderlyingLayerEvent::TimerExpired(timer_kind) => {
                // Handle timer expiration from underlying layer
                #[cfg(target_arch = "wasm32")]
                log!("Timer expired event received: {}", timer_kind);
                #[cfg(not(target_arch = "wasm32"))]
                println!("Timer expired event received: {}", timer_kind);

                // Check if connection is closed - if so, ignore all timers
                if matches!(self.state, ConnectionState::Closed) {
                    #[cfg(target_arch = "wasm32")]
                    log!("Connection closed, ignoring timer: {}", timer_kind);
                    #[cfg(not(target_arch = "wasm32"))]
                    println!("Connection closed, ignoring timer: {}", timer_kind);
                    return;
                }

                // Check if timer was cancelled - if so, ignore the expiration
                if !self.active_timers.contains(&timer_kind) {
                    #[cfg(target_arch = "wasm32")]
                    log!("Timer {} was cancelled, ignoring expiration", timer_kind);
                    #[cfg(not(target_arch = "wasm32"))]
                    println!("Timer {} was cancelled, ignoring expiration", timer_kind);
                    return;
                }

                // Remove the timer from active_timers since it has now expired
                self.active_timers.remove(&timer_kind);
                #[cfg(target_arch = "wasm32")]
                log!(
                    "Timer {} expired and removed from active timers",
                    timer_kind
                );
                #[cfg(not(target_arch = "wasm32"))]
                println!(
                    "Timer {} expired and removed from active timers",
                    timer_kind
                );

                // Handle timer expiration based on timer type string
                if timer_kind.contains("PingreqSend") {
                    let events = self
                        .mqtt_connection
                        .notify_timer_fired(mqtt::connection::TimerKind::PingreqSend);
                    let _ = self.handle_mqtt_events(events);
                } else if timer_kind.contains("PingrespRecv") {
                    let events = self
                        .mqtt_connection
                        .notify_timer_fired(mqtt::connection::TimerKind::PingrespRecv);
                    let _ = self.handle_mqtt_events(events);
                } else {
                    #[cfg(target_arch = "wasm32")]
                    log!("Unknown timer kind: {}", timer_kind);
                    #[cfg(not(target_arch = "wasm32"))]
                    println!("Unknown timer kind: {}", timer_kind);
                }
            }
        }
    }

    /// Handle received packet - try to deliver to pending recv requests
    /// If delivery fails (receiver dropped due to timeout), save packet for next recv()
    fn handle_received_packet(&mut self, packet: mqtt::packet::Packet) {
        // Try to deliver packet to pending recv requests
        // If receiver is dropped (timeout), try next request
        let mut packet_to_deliver = Some(packet);

        while let Some(pkt) = packet_to_deliver.take() {
            if self.pending_recv_requests.is_empty() {
                // No more requests, save packet for next recv() call
                self.undelivered_packet = Some(pkt);
                break;
            }

            // Try to deliver to the first (oldest) pending request
            let reply = self.pending_recv_requests.remove(0);
            match reply.send(Ok(pkt)) {
                Ok(()) => {
                    // Successfully delivered packet
                    break;
                }
                Err(packet_result) => {
                    // Receiver dropped (e.g., due to timeout)
                    // Get packet back and try next request
                    if let Ok(returned_packet) = packet_result {
                        packet_to_deliver = Some(returned_packet);
                    }
                    // Continue loop to try next request
                }
            }
        }
    }

    /// Connect WebSocket to MQTT broker
    async fn connect(&mut self, url: &str, reply: oneshot::Sender<Result<()>>) -> Result<()> {
        // Allow connection from Disconnected or Closed states (for reconnection support)
        if self.state == ConnectionState::Connecting || self.state == ConnectionState::Connected {
            let _ = reply.send(Err(Error::Other(
                "Already connecting or connected".to_string(),
            )));
            return Ok(());
        }

        // Reset internal state for reconnection
        if self.state == ConnectionState::Closed {
            self.reset_for_reconnection();
        }

        self.state = ConnectionState::Connecting;
        let reply_arc = std::sync::Arc::new(std::sync::Mutex::new(Some(reply)));

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Sending Connect command with reply_arc".into());

        let _ = self
            .websocket_commands
            .unbounded_send(UnderlyingLayerCommand::Connect(url.to_string(), reply_arc));

        Ok(())
    }

    /// Send MQTT packet
    async fn send_packet(&mut self, packet: mqtt::packet::Packet) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Sending MQTT packet: {:?}", packet).into());
        let events = self.mqtt_connection.send(packet);
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("MQTT send returned {} events", events.len()).into());
        self.handle_mqtt_events(events)
    }

    /// Close connection
    async fn close(&mut self) -> Result<()> {
        let _ = self
            .websocket_commands
            .unbounded_send(UnderlyingLayerCommand::Close);
        self.state = ConnectionState::Closed;
        Ok(())
    }

    /// Process incoming WebSocket data
    fn process_incoming_data(&mut self, data: Vec<u8>) {
        // Append to buffer
        let new_data_len = data.len();

        // Compact buffer if needed
        if self.consumed_bytes > 0 {
            let unconsumed_len = self.buffer_size - self.consumed_bytes;
            if unconsumed_len > 0 {
                self.read_buffer
                    .copy_within(self.consumed_bytes..self.buffer_size, 0);
            }
            self.buffer_size = unconsumed_len;
            self.consumed_bytes = 0;
        }

        // Ensure capacity and copy data
        self.read_buffer.resize(self.buffer_size + new_data_len, 0);
        self.read_buffer[self.buffer_size..self.buffer_size + new_data_len].copy_from_slice(&data);
        self.buffer_size += new_data_len;

        // Process buffer
        if self.consumed_bytes < self.buffer_size {
            let unconsumed_data = &self.read_buffer[self.consumed_bytes..self.buffer_size];
            let mut cursor = mqtt::common::Cursor::new(unconsumed_data);

            let events = self.mqtt_connection.recv(&mut cursor);
            self.consumed_bytes += cursor.position() as usize;

            let _ = self.handle_mqtt_events(events);
        }
    }

    /// Handle MQTT events (store send requests for async processing)
    #[allow(unused_variables)]
    fn handle_mqtt_events(&mut self, events: Vec<mqtt::connection::Event>) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &format!(
                "🚀🚀🚀 FORCE UPDATE: Processing {} MQTT events 🚀🚀🚀",
                events.len()
            )
            .into(),
        );

        for (i, event) in events.iter().enumerate() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Event {}: {:?}", i + 1, event).into());
        }

        for event in events {
            match event {
                mqtt::connection::Event::RequestSendPacket { packet, .. } => {
                    let buffer = packet.to_continuous_buffer();
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(
                        &format!("Sending packet: {} bytes", buffer.len()).into(),
                    );
                    // Send via WebSocket command
                    match self
                        .websocket_commands
                        .unbounded_send(UnderlyingLayerCommand::SendData(buffer))
                    {
                        Ok(_) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(
                                &"Sent packet successfully via WebSocket command".into(),
                            );
                        }
                        Err(e) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(
                                &format!("Failed to send packet via WebSocket command: {:?}", e)
                                    .into(),
                            );
                        }
                    }
                }
                mqtt::connection::Event::NotifyPacketReceived(packet) => {
                    if self.packet_sender.unbounded_send(packet).is_err() {
                        eprintln!("Failed to forward received packet");
                    }
                }
                mqtt::connection::Event::RequestTimerReset { kind, duration_ms } => {
                    let kind_str = format!("{:?}", kind);

                    // Track the timer as active
                    self.active_timers.insert(kind_str.clone());

                    // Delegate timer handling to underlying layer
                    // The underlying layer will send TimerExpired event when the timer fires
                    let _ = self.websocket_commands.unbounded_send(
                        UnderlyingLayerCommand::TimerReset {
                            kind: kind_str,
                            duration_ms,
                        },
                    );
                }
                mqtt::connection::Event::RequestTimerCancel(kind) => {
                    let kind_str = format!("{:?}", kind);

                    // Remove from tracking
                    self.active_timers.remove(&kind_str);

                    // Delegate cancellation to underlying layer
                    let _ = self
                        .websocket_commands
                        .unbounded_send(UnderlyingLayerCommand::TimerCancel { kind: kind_str });
                }
                mqtt::connection::Event::NotifyError(error) => {
                    eprintln!("MQTT protocol error: {:?}", error);
                }
                mqtt::connection::Event::RequestClose => {
                    let _ = self
                        .websocket_commands
                        .unbounded_send(UnderlyingLayerCommand::Close);
                    self.state = ConnectionState::Closed;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Reset internal state for reconnection
    /// Called when attempting to connect from Closed state
    fn reset_for_reconnection(&mut self) {
        #[cfg(target_arch = "wasm32")]
        log!("Resetting internal state for reconnection");

        // Reset MQTT connection (create new connection with same version)
        self.mqtt_connection = mqtt::Connection::<mqtt::role::Client>::new(self.config.version);

        // Reconfigure MQTT connection
        if let Some(interval) = self.config.pingreq_send_interval_ms {
            self.mqtt_connection
                .set_pingreq_send_interval(Some(interval));
        }
        self.mqtt_connection
            .set_auto_pub_response(self.config.auto_pub_response);
        self.mqtt_connection
            .set_auto_ping_response(self.config.auto_ping_response);
        self.mqtt_connection
            .set_auto_map_topic_alias_send(self.config.auto_map_topic_alias_send);
        self.mqtt_connection
            .set_auto_replace_topic_alias_send(self.config.auto_replace_topic_alias_send);
        self.mqtt_connection
            .set_pingresp_recv_timeout(self.config.pingresp_recv_timeout_ms);

        // Clear buffers
        self.read_buffer.clear();
        self.buffer_size = 0;
        self.consumed_bytes = 0;

        // Clear pending recv requests (they should have been cleaned up, but just in case)
        self.pending_recv_requests.clear();
        self.undelivered_packet = None;

        // Clear timers (should already be cleared on close, but ensure it)
        self.active_timers.clear();

        #[cfg(target_arch = "wasm32")]
        log!("Internal state reset complete for reconnection");
    }
}

impl MqttClient {
    /// Create new MQTT client with configuration
    #[cfg(target_arch = "wasm32")]
    pub fn new(config: MqttConfig) -> Self {
        Self::new_with_websocket(config, BrowserWebSocket::new())
    }

    /// Create new MQTT client with custom WebSocket (for testing)
    #[cfg(target_arch = "wasm32")]
    pub fn new_with_websocket<W: UnderlyingLayerInterface + 'static>(
        config: MqttConfig,
        websocket: W,
    ) -> Self {
        let (request_sender, request_receiver) = mpsc::unbounded();

        // Start background processor
        let (mut processor, mut websocket) =
            MqttProcessor::new(config, websocket, request_sender.clone());

        use wasm_bindgen_futures::spawn_local;

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Starting WebSocket processor task".into());

        // Start WebSocket processor
        spawn_local(async move {
            websocket.run().await;
        });

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Starting MQTT processor task".into());

        // Start MQTT processor
        spawn_local(async move {
            processor.run(request_receiver).await;
        });

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &"★★★ CLIENT_CLEAN: Both processors started, returning client ★★★".into(),
        );

        Self { request_sender }
    }

    /// Create new MQTT client with custom WebSocket (for testing, non-WASM)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_with_websocket<W: UnderlyingLayerInterface + Send + 'static>(
        config: MqttConfig,
        websocket: W,
    ) -> Self {
        let (request_sender, request_receiver) = mpsc::unbounded();

        // Start background processor
        let (mut processor, mut websocket) =
            MqttProcessor::new(config, websocket, request_sender.clone());

        // Start WebSocket processor with tokio runtime for timer support
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(websocket.run());
        });

        // Start MQTT processor with tokio runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(processor.run(request_receiver));
        });

        Self { request_sender }
    }

    /// Connect to MQTT broker
    pub async fn connect(&self, url: &str) -> Result<()> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::Connect {
            url: url.to_string(),
            reply: reply_sender,
        };

        self.request_sender
            .unbounded_send(request)
            .map_err(|_| Error::Other("Client channel closed".to_string()))?;

        reply_receiver
            .await
            .map_err(|_| Error::Other("Request cancelled".to_string()))?
    }

    /// Send MQTT packet
    pub async fn send(&self, packet: mqtt::packet::Packet) -> Result<()> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::Send {
            packet,
            reply: reply_sender,
        };

        self.request_sender
            .unbounded_send(request)
            .map_err(|_| Error::Other("Client channel closed".to_string()))?;

        reply_receiver
            .await
            .map_err(|_| Error::Other("Request cancelled".to_string()))?
    }

    /// Receive MQTT packet
    pub async fn recv(&self) -> Result<mqtt::packet::Packet> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::Recv {
            reply: reply_sender,
        };

        self.request_sender
            .unbounded_send(request)
            .map_err(|_| Error::Other("Client channel closed".to_string()))?;

        reply_receiver
            .await
            .map_err(|_| Error::Other("Request cancelled".to_string()))?
    }

    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::Close {
            reply: reply_sender,
        };

        self.request_sender
            .unbounded_send(request)
            .map_err(|_| Error::Other("Client channel closed".to_string()))?;

        reply_receiver
            .await
            .map_err(|_| Error::Other("Request cancelled".to_string()))?
    }

    /// Get current connection state
    pub async fn state(&self) -> ConnectionState {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::State {
            reply: reply_sender,
        };

        if self.request_sender.unbounded_send(request).is_err() {
            return ConnectionState::Closed;
        }

        reply_receiver.await.unwrap_or(ConnectionState::Closed)
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::IsConnected {
            reply: reply_sender,
        };

        if self.request_sender.unbounded_send(request).is_err() {
            return false;
        }

        reply_receiver.await.unwrap_or(false)
    }

    /// Acquire a packet ID for use with QoS 1 or 2 messages
    pub async fn acquire_packet_id(&self) -> Option<u16> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::AcquirePacketId {
            reply: reply_sender,
        };

        if self.request_sender.unbounded_send(request).is_err() {
            return None;
        }

        reply_receiver.await.unwrap_or(None)
    }

    /// Register a packet ID as in use
    pub async fn register_packet_id(&self, packet_id: u16) -> bool {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::RegisterPacketId {
            packet_id,
            reply: reply_sender,
        };

        if self.request_sender.unbounded_send(request).is_err() {
            return false;
        }

        reply_receiver.await.unwrap_or(false)
    }

    /// Release a packet ID
    pub async fn release_packet_id(&self, packet_id: u16) -> Result<()> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let request = Request::ReleasePacketId {
            packet_id,
            reply: reply_sender,
        };

        self.request_sender
            .unbounded_send(request)
            .map_err(|_| Error::Other("Client channel closed".to_string()))?;

        reply_receiver
            .await
            .map_err(|_| Error::Other("Request cancelled".to_string()))?
    }
}
