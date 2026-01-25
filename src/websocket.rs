//! WebSocket abstraction layer for testability

use async_trait::async_trait;
use futures::channel::{mpsc, oneshot};

/// Type alias for the connect reply sender
pub type ConnectReplySender =
    std::sync::Arc<std::sync::Mutex<Option<oneshot::Sender<Result<(), crate::error::Error>>>>>;

/// WebSocket events
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    Connected,
    Message(Vec<u8>),
    Error(String),
    Closed,
}

/// WebSocket commands (sent TO the WebSocket from message loop)
#[derive(Debug, Clone)]
pub enum WebSocketCommand {
    Connect(String, ConnectReplySender),
    SendData(Vec<u8>),
    Close,
    TimerExpired(String),
}

/// Abstract WebSocket interface for testing (pure message-passing)
#[async_trait(?Send)]
#[cfg(target_arch = "wasm32")]
pub trait WebSocketInterface {
    /// Get event receiver (events FROM WebSocket TO message loop)
    fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<WebSocketEvent>;

    /// Get command sender (commands TO WebSocket FROM message loop)
    fn command_sender(&self) -> mpsc::UnboundedSender<WebSocketCommand>;

    /// Start the WebSocket processor (handles commands and generates events)
    async fn run(&mut self);
}

/// Abstract WebSocket interface for testing (pure message-passing)
#[async_trait(?Send)]
#[cfg(not(target_arch = "wasm32"))]
pub trait WebSocketInterface: Send {
    /// Get event receiver (events FROM WebSocket TO message loop)
    fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<WebSocketEvent>;

    /// Get command sender (commands TO WebSocket FROM message loop)
    fn command_sender(&self) -> mpsc::UnboundedSender<WebSocketCommand>;

    /// Start the WebSocket processor (handles commands and generates events)
    async fn run(&mut self);
}

/// Browser WebSocket implementation (pure message-passing)
#[cfg(target_arch = "wasm32")]
pub struct BrowserWebSocket {
    event_sender: mpsc::UnboundedSender<WebSocketEvent>,
    event_receiver: Option<mpsc::UnboundedReceiver<WebSocketEvent>>,
    command_sender: mpsc::UnboundedSender<WebSocketCommand>,
    command_receiver: mpsc::UnboundedReceiver<WebSocketCommand>,
}

#[cfg(target_arch = "wasm32")]
impl BrowserWebSocket {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded();
        let (command_sender, command_receiver) = mpsc::unbounded();

        Self {
            event_sender,
            event_receiver: Some(event_receiver),
            command_sender,
            command_receiver,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl WebSocketInterface for BrowserWebSocket {
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
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::{BinaryType, ErrorEvent, MessageEvent, WebSocket};

        web_sys::console::log_1(&"🚀 WEBSOCKET: NEW VERSION - CONNECT TIMING FIXED 🚀".into());

        let mut websocket: Option<web_sys::WebSocket> = None;
        let mut _closures: Vec<wasm_bindgen::closure::Closure<dyn FnMut(wasm_bindgen::JsValue)>> =
            Vec::new();
        let _is_connected = false;
        let _pending_data: Vec<Vec<u8>> = Vec::new();

        web_sys::console::log_1(&"WebSocket processor waiting for commands".into());

        while let Some(command) = self.command_receiver.next().await {
            web_sys::console::log_1(
                &format!("WebSocket processor received command: {:?}", command).into(),
            );
            match command {
                WebSocketCommand::Connect(url, reply_arc) => {
                    web_sys::console::log_1(&format!("WebSocket connecting to: {}", url).into());
                    web_sys::console::log_1(&"✅ Received Connect command with reply_arc".into());

                    // MQTT subprotocol is required
                    let protocols = js_sys::Array::new();
                    protocols.push(&wasm_bindgen::JsValue::from_str("mqtt"));
                    let ws_result = WebSocket::new_with_str_sequence(&url, &protocols);

                    match ws_result {
                        Ok(ws) => {
                            web_sys::console::log_1(&"WebSocket created successfully".into());
                            ws.set_binary_type(BinaryType::Arraybuffer);
                            web_sys::console::log_1(&"Binary type set to ArrayBuffer".into());

                            let event_sender = self.event_sender.clone();
                            web_sys::console::log_1(&"Event sender cloned for closures".into());

                            // onopen
                            let event_sender_clone = event_sender.clone();
                            let reply_arc_clone = reply_arc.clone();
                            web_sys::console::log_1(&"Creating onopen closure".into());
                            let onopen = Closure::wrap(Box::new(move |_: JsValue| {
                                web_sys::console::log_1(
                                    &"🔥 NEW WEBSOCKET: WebSocket onopen fired 🔥".into(),
                                );

                                // Send reply to complete the connect() await
                                web_sys::console::log_1(
                                    &"Attempting to lock reply_arc in onopen".into(),
                                );
                                match reply_arc_clone.lock() {
                                    Ok(mut reply_opt) => {
                                        web_sys::console::log_1(
                                            &"Successfully locked reply_arc".into(),
                                        );
                                        if let Some(reply) = reply_opt.take() {
                                            match reply.send(Ok(())) {
                                                Ok(_) => web_sys::console::log_1(&"✅ Sent connect completion reply successfully".into()),
                                                Err(_) => web_sys::console::log_1(&"❌ Failed to send connect completion reply - receiver dropped".into()),
                                            }
                                        } else {
                                            web_sys::console::log_1(
                                                &"❌ No reply sender in Option".into(),
                                            );
                                        }
                                    }
                                    Err(_) => {
                                        web_sys::console::log_1(
                                            &"❌ Failed to lock reply_arc".into(),
                                        );
                                    }
                                }

                                match event_sender_clone.unbounded_send(WebSocketEvent::Connected) {
                                    Ok(_) => web_sys::console::log_1(
                                        &"Sent Connected event successfully".into(),
                                    ),
                                    Err(e) => web_sys::console::log_1(
                                        &format!("Failed to send Connected event: {:?}", e).into(),
                                    ),
                                }
                            })
                                as Box<dyn FnMut(JsValue)>);
                            web_sys::console::log_1(
                                &"onopen closure created, setting on WebSocket".into(),
                            );
                            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                            web_sys::console::log_1(
                                &"onopen set on WebSocket, pushing to closures vec".into(),
                            );
                            _closures.push(onopen);
                            web_sys::console::log_1(&"onopen closure pushed to vec".into());

                            // onmessage
                            let event_sender_clone = event_sender.clone();
                            web_sys::console::log_1(&"Creating onmessage closure".into());
                            let onmessage = Closure::wrap(Box::new(move |e: JsValue| {
                                web_sys::console::log_1(&"WebSocket onmessage fired".into());
                                let event: MessageEvent = e.dyn_into().unwrap();
                                if let Ok(array_buffer) =
                                    event.data().dyn_into::<js_sys::ArrayBuffer>()
                                {
                                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                                    let mut data = vec![0; uint8_array.length() as usize];
                                    uint8_array.copy_to(&mut data);
                                    web_sys::console::log_1(
                                        &format!("Received {} bytes", data.len()).into(),
                                    );
                                    match event_sender_clone
                                        .unbounded_send(WebSocketEvent::Message(data))
                                    {
                                        Ok(_) => web_sys::console::log_1(
                                            &"Sent Message event successfully".into(),
                                        ),
                                        Err(e) => web_sys::console::log_1(
                                            &format!("Failed to send Message event: {:?}", e)
                                                .into(),
                                        ),
                                    }
                                }
                            })
                                as Box<dyn FnMut(JsValue)>);
                            web_sys::console::log_1(
                                &"onmessage closure created, setting on WebSocket".into(),
                            );
                            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                            web_sys::console::log_1(
                                &"onmessage set on WebSocket, pushing to closures vec".into(),
                            );
                            _closures.push(onmessage);
                            web_sys::console::log_1(&"onmessage closure pushed to vec".into());

                            // onerror - with detailed logging
                            let event_sender_clone = event_sender.clone();
                            let onerror = Closure::wrap(Box::new(move |e: JsValue| {
                                web_sys::console::log_1(&"WebSocket onerror fired".into());
                                web_sys::console::log_1(&format!("Error event: {:?}", e).into());

                                let error_msg = if let Ok(error_event) = e.dyn_into::<ErrorEvent>()
                                {
                                    let msg = error_event.message();
                                    web_sys::console::log_1(
                                        &format!("ErrorEvent message: {}", msg).into(),
                                    );
                                    msg
                                } else {
                                    web_sys::console::log_1(
                                        &"Not an ErrorEvent - unknown error".into(),
                                    );
                                    "Unknown WebSocket error".to_string()
                                };
                                let _ = event_sender_clone
                                    .unbounded_send(WebSocketEvent::Error(error_msg));
                            })
                                as Box<dyn FnMut(JsValue)>);
                            ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                            _closures.push(onerror);

                            // onclose
                            let event_sender_clone = event_sender.clone();
                            let onclose = Closure::wrap(Box::new(move |e: JsValue| {
                                web_sys::console::log_1(&"WebSocket onclose fired".into());
                                web_sys::console::log_1(&format!("Close event: {:?}", e).into());

                                // Try to get close details
                                if let Ok(close_event) = e.dyn_into::<web_sys::CloseEvent>() {
                                    let code = close_event.code();
                                    let reason = close_event.reason();
                                    let was_clean = close_event.was_clean();
                                    web_sys::console::log_1(
                                        &format!(
                                            "Close code: {}, reason: '{}', clean: {}",
                                            code, reason, was_clean
                                        )
                                        .into(),
                                    );
                                }

                                let _ = event_sender_clone.unbounded_send(WebSocketEvent::Closed);
                            })
                                as Box<dyn FnMut(JsValue)>);
                            ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                            _closures.push(onclose);

                            websocket = Some(ws);
                        }
                        Err(e) => {
                            let _ =
                                self.event_sender
                                    .unbounded_send(WebSocketEvent::Error(format!(
                                        "Failed to create WebSocket: {:?}",
                                        e
                                    )));
                        }
                    }
                }
                WebSocketCommand::SendData(data) => {
                    web_sys::console::log_1(
                        &format!("WebSocket SendData command: {} bytes", data.len()).into(),
                    );
                    if let Some(ref ws) = websocket {
                        web_sys::console::log_1(
                            &"WebSocket is available, attempting to send".into(),
                        );
                        match ws.send_with_u8_array(&data) {
                            Ok(_) => {
                                web_sys::console::log_1(
                                    &"WebSocket send_with_u8_array succeeded".into(),
                                );
                            }
                            Err(e) => {
                                web_sys::console::log_1(
                                    &format!("WebSocket send_with_u8_array failed: {:?}", e).into(),
                                );
                                let _ = self.event_sender.unbounded_send(WebSocketEvent::Error(
                                    format!("Send failed: {:?}", e),
                                ));
                            }
                        }
                    } else {
                        web_sys::console::log_1(&"WebSocket not available for sending".into());
                        let _ = self.event_sender.unbounded_send(WebSocketEvent::Error(
                            "WebSocket not connected".to_string(),
                        ));
                    }
                }
                WebSocketCommand::Close => {
                    // Clear closures first to prevent further callbacks
                    _closures.clear();

                    if let Some(ws) = websocket.take() {
                        // Remove event handlers before closing
                        ws.set_onopen(None);
                        ws.set_onmessage(None);
                        ws.set_onerror(None);
                        ws.set_onclose(None);
                        let _ = ws.close();
                    }

                    let _ = self.event_sender.unbounded_send(WebSocketEvent::Closed);
                    // Do NOT break - allow reconnection by continuing to process commands
                }
                WebSocketCommand::TimerExpired(timer_kind) => {
                    // Timer expired events will be handled by main processor
                    web_sys::console::log_1(
                        &format!("WebSocket processor received timer expired: {}", timer_kind)
                            .into(),
                    );
                }
            }
        }
    }
}
