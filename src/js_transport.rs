//! JavaScript Transport Bridge
//!
//! This module provides a bridge between JavaScript transports (TCP, TLS, WebSocket)
//! and the Rust WASM client. It implements UnderlyingLayerInterface so that any JavaScript
//! transport can be used with the MqttClient's state machine and timers.

use crate::websocket::{
    ConnectReplySender, UnderlyingLayerCommand, UnderlyingLayerEvent, UnderlyingLayerInterface,
};
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// JavaScript transport callbacks interface
///
/// JavaScript must implement these callbacks to handle transport operations:
/// - onSend: called when WASM needs to send data
/// - onClose: called when WASM requests connection close
#[wasm_bindgen]
extern "C" {
    /// JavaScript transport object type
    #[wasm_bindgen(typescript_type = "JsTransportCallbacks")]
    pub type JsTransportCallbacks;

    /// Called by WASM to send data via the transport
    #[wasm_bindgen(method, js_name = onSend)]
    pub fn on_send(this: &JsTransportCallbacks, data: &[u8]);

    /// Called by WASM to close the transport
    #[wasm_bindgen(method, js_name = onClose)]
    pub fn on_close(this: &JsTransportCallbacks);
}

/// Shared state for JavaScript transport bridge
/// This is wrapped in Rc<RefCell<>> so it can be shared between the WASM side and JavaScript side
struct SharedTransportState {
    event_sender: mpsc::UnboundedSender<UnderlyingLayerEvent>,
    js_callbacks: Option<JsTransportCallbacks>,
    connect_reply: Option<ConnectReplySender>,
}

/// Transport handle that implements UnderlyingLayerInterface
/// This is the part that gets used by MqttClient internally
pub struct JsTransportHandle {
    shared: Rc<RefCell<SharedTransportState>>,
    event_receiver: Option<mpsc::UnboundedReceiver<UnderlyingLayerEvent>>,
    command_sender: mpsc::UnboundedSender<UnderlyingLayerCommand>,
    command_receiver: Option<mpsc::UnboundedReceiver<UnderlyingLayerCommand>>,
}

impl JsTransportHandle {
    fn new(shared: Rc<RefCell<SharedTransportState>>) -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded();
        let (command_sender, command_receiver) = mpsc::unbounded();

        // Update the shared state with the new event sender
        shared.borrow_mut().event_sender = event_sender;

        JsTransportHandle {
            shared,
            event_receiver: Some(event_receiver),
            command_sender,
            command_receiver: Some(command_receiver),
        }
    }
}

#[async_trait(?Send)]
impl UnderlyingLayerInterface for JsTransportHandle {
    fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<UnderlyingLayerEvent> {
        self.event_receiver.take().unwrap_or_else(|| {
            let (_, receiver) = mpsc::unbounded();
            receiver
        })
    }

    fn command_sender(&self) -> mpsc::UnboundedSender<UnderlyingLayerCommand> {
        self.command_sender.clone()
    }

    async fn run(&mut self) {
        let shared = self.shared.clone();
        let mut command_receiver = self.command_receiver.take().unwrap_or_else(|| {
            let (_, receiver) = mpsc::unbounded();
            receiver
        });

        while let Some(command) = command_receiver.next().await {
            match command {
                UnderlyingLayerCommand::Connect(_url, reply_arc) => {
                    // Store the reply for when JavaScript calls notifyConnected
                    shared.borrow_mut().connect_reply = Some(reply_arc);
                }
                UnderlyingLayerCommand::SendData(data) => {
                    let shared_borrowed = shared.borrow();
                    if let Some(ref callbacks) = shared_borrowed.js_callbacks {
                        callbacks.on_send(&data);
                    }
                }
                UnderlyingLayerCommand::Close => {
                    let shared_borrowed = shared.borrow();
                    if let Some(ref callbacks) = shared_borrowed.js_callbacks {
                        callbacks.on_close();
                    }
                }
                UnderlyingLayerCommand::TimerReset { kind, duration_ms } => {
                    // Timer handling for JsTransport should be done in JavaScript
                    // For now, just log the request
                    web_sys::console::log_1(
                        &format!("JsTransport: TimerReset {} for {}ms", kind, duration_ms).into(),
                    );
                }
                UnderlyingLayerCommand::TimerCancel { kind } => {
                    // Timer handling for JsTransport should be done in JavaScript
                    // For now, just log the request
                    web_sys::console::log_1(&format!("JsTransport: TimerCancel {}", kind).into());
                }
            }
        }
    }
}

/// JavaScript Transport Bridge
///
/// This struct bridges JavaScript transport implementations to the Rust UnderlyingLayerInterface.
/// It allows Node.js transports (TCP, TLS, WebSocket) to integrate with the WASM client's
/// state machine, timers, and automatic packet handling.
///
/// The JsTransport stays in JavaScript and can be used to notify events.
/// A separate JsTransportHandle is created for use by the Rust client.
#[wasm_bindgen]
pub struct JsTransport {
    shared: Rc<RefCell<SharedTransportState>>,
}

#[wasm_bindgen]
impl JsTransport {
    /// Create a new JavaScript transport bridge
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsTransport {
        // Create a dummy sender that will be replaced when createHandle is called
        let (event_sender, _) = mpsc::unbounded();

        let shared = Rc::new(RefCell::new(SharedTransportState {
            event_sender,
            js_callbacks: None,
            connect_reply: None,
        }));

        JsTransport { shared }
    }

    /// Set the JavaScript callbacks for transport operations
    ///
    /// The callbacks object must implement:
    /// - onSend(data: Uint8Array): void - called to send data via transport
    /// - onClose(): void - called to close the transport
    #[wasm_bindgen(js_name = setCallbacks)]
    pub fn set_callbacks(&mut self, callbacks: JsTransportCallbacks) {
        self.shared.borrow_mut().js_callbacks = Some(callbacks);
    }

    /// Called by JavaScript when the transport connects successfully
    #[wasm_bindgen(js_name = notifyConnected)]
    pub fn notify_connected(&self) {
        let shared = self.shared.borrow();

        // Complete the connect await if there's a pending reply
        if let Some(ref reply_arc) = shared.connect_reply {
            if let Ok(mut reply_opt) = reply_arc.lock() {
                if let Some(reply) = reply_opt.take() {
                    let _ = reply.send(Ok(()));
                }
            }
        }

        let _ = shared
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Connected);
    }

    /// Called by JavaScript when data is received from the transport
    #[wasm_bindgen(js_name = notifyMessage)]
    pub fn notify_message(&self, data: &[u8]) {
        let shared = self.shared.borrow();
        let _ = shared
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Message(data.to_vec()));
    }

    /// Called by JavaScript when an error occurs
    #[wasm_bindgen(js_name = notifyError)]
    pub fn notify_error(&self, error: &str) {
        let shared = self.shared.borrow();

        // If we have a pending connect, complete it with error
        if let Some(ref reply_arc) = shared.connect_reply {
            if let Ok(mut reply_opt) = reply_arc.lock() {
                if let Some(reply) = reply_opt.take() {
                    let _ = reply.send(Err(crate::error::Error::Other(error.to_string())));
                }
            }
        }

        let _ = shared
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Error(error.to_string()));
    }

    /// Called by JavaScript when the transport is closed
    #[wasm_bindgen(js_name = notifyClosed)]
    pub fn notify_closed(&self) {
        let shared = self.shared.borrow();
        let _ = shared
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Closed);
    }
}

impl Default for JsTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl JsTransport {
    /// Create a transport handle for use by MqttClient
    /// This is an internal method, not exposed to JavaScript
    pub fn create_handle(&self) -> JsTransportHandle {
        JsTransportHandle::new(self.shared.clone())
    }
}

/// Create a WasmMqttClient with a JsTransport
/// This is a helper function that properly sets up the transport handle
#[wasm_bindgen(js_name = createClientWithJsTransport)]
pub fn create_client_with_js_transport(
    config: crate::wasm_interface::WasmMqttConfig,
    transport: &JsTransport,
) -> crate::wasm_interface::WasmMqttClient {
    web_sys::console::log_1(&"Creating WasmMqttClient with JsTransport handle".into());
    let handle = transport.create_handle();
    let version = config.version();
    let inner_config = config.into_inner();
    let client = crate::MqttClient::new_with_websocket(inner_config, handle);
    crate::wasm_interface::WasmMqttClient::from_client(client, version)
}
