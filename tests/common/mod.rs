//! Common test utilities
//!
//! This module provides test helpers like MockWebSocket for integration tests.

use async_trait::async_trait;
use futures::channel::mpsc;
use mqtt_client_wasm::{WebSocketCommand, WebSocketEvent, WebSocketInterface};

/// Mock WebSocket for testing (pure message-passing)
pub struct MockWebSocket {
    pub event_sender: mpsc::UnboundedSender<WebSocketEvent>,
    event_receiver: Option<mpsc::UnboundedReceiver<WebSocketEvent>>,
    command_sender: mpsc::UnboundedSender<WebSocketCommand>,
    command_receiver: mpsc::UnboundedReceiver<WebSocketCommand>,
    connected: bool,
    sent_data: Vec<Vec<u8>>,
}

#[allow(dead_code)]
impl MockWebSocket {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded();
        let (command_sender, command_receiver) = mpsc::unbounded();

        Self {
            event_sender,
            event_receiver: Some(event_receiver),
            command_sender,
            command_receiver,
            connected: false,
            sent_data: Vec::new(),
        }
    }

    /// Simulate receiving data
    pub fn simulate_receive(&self, data: Vec<u8>) {
        let _ = self
            .event_sender
            .unbounded_send(WebSocketEvent::Message(data));
    }

    /// Simulate connection
    pub fn simulate_connect(&mut self) {
        self.connected = true;
        let _ = self.event_sender.unbounded_send(WebSocketEvent::Connected);
    }

    /// Simulate error
    pub fn simulate_error(&self, error: String) {
        let _ = self
            .event_sender
            .unbounded_send(WebSocketEvent::Error(error));
    }

    /// Simulate close
    pub fn simulate_close(&mut self) {
        self.connected = false;
        let _ = self.event_sender.unbounded_send(WebSocketEvent::Closed);
    }

    /// Get sent data for verification
    pub fn sent_data(&self) -> &[Vec<u8>] {
        &self.sent_data
    }
}

#[async_trait(?Send)]
impl WebSocketInterface for MockWebSocket {
    fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<WebSocketEvent> {
        self.event_receiver.take().unwrap_or_else(|| {
            let (_, receiver) = mpsc::unbounded();
            receiver
        })
    }

    fn command_sender(&self) -> mpsc::UnboundedSender<WebSocketCommand> {
        self.command_sender.clone()
    }

    async fn run(&mut self) {
        use futures::stream::StreamExt;

        while let Some(command) = self.command_receiver.next().await {
            match command {
                WebSocketCommand::Connect(url, reply_arc) => {
                    println!("MockWebSocket connecting to: {}", url);
                    self.connected = true;

                    // Send reply to complete the connect() await
                    if let Ok(mut reply_opt) = reply_arc.lock() {
                        if let Some(reply) = reply_opt.take() {
                            let _ = reply.send(Ok(()));
                        }
                    }

                    let _ = self.event_sender.unbounded_send(WebSocketEvent::Connected);
                }
                WebSocketCommand::SendData(data) => {
                    if self.connected {
                        self.sent_data.push(data);
                    } else {
                        let _ = self
                            .event_sender
                            .unbounded_send(WebSocketEvent::Error("Not connected".to_string()));
                    }
                }
                WebSocketCommand::Close => {
                    self.connected = false;
                    let _ = self.event_sender.unbounded_send(WebSocketEvent::Closed);
                    // Don't break - allow reconnection by continuing to process commands
                }
                WebSocketCommand::TimerExpired(timer_kind) => {
                    // MockWebSocket ignores timer events
                    println!("MockWebSocket received timer expired: {}", timer_kind);
                }
            }
        }
    }
}
