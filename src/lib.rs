//! MQTT Client for WASM
//!
//! This library provides a WebSocket-based MQTT client that works in WASM environments.
//! It offers a low-level endpoint API similar to mqtt-endpoint-tokio, providing
//! basic operations like send, recv, and close without high-level publish/subscribe abstractions.

mod client;
mod error;
pub mod platform;
mod types;
mod websocket;

#[cfg(target_arch = "wasm32")]
mod js_transport;
#[cfg(target_arch = "wasm32")]
mod wasm_interface;

pub use client::MqttClient;
pub use error::{Error, Result};
pub use types::*;
pub use websocket::{WebSocketCommand, WebSocketEvent, WebSocketInterface};

// WASM-specific exports - export the clean client implementation
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    pub use crate::js_transport::{create_client_with_js_transport, JsTransport};
    pub use crate::wasm_interface::{
        WasmMqttClient, WasmMqttConfig, WasmMqttPacket, WasmPacketType,
    };
    pub use crate::{MqttClient, MqttConfig};
}

// Re-export organized MQTT types
pub mod mqtt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
