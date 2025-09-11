//! Common types and configuration

use mqtt_protocol_core::mqtt;

/// MQTT client configuration
#[derive(Debug, Clone)]
pub struct MqttConfig {
    pub url: String,
    pub version: mqtt::Version,
    pub pingreq_send_interval_ms: Option<u64>,
    pub auto_pub_response: bool,
    pub auto_ping_response: bool,
    pub auto_map_topic_alias_send: bool,
    pub auto_replace_topic_alias_send: bool,
    pub pingresp_recv_timeout_ms: u64,
    pub connection_establish_timeout_ms: u64,
    pub shutdown_timeout_ms: u64,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            version: mqtt::Version::V5_0,
            pingreq_send_interval_ms: None,
            auto_pub_response: true,
            auto_ping_response: true,
            auto_map_topic_alias_send: false,
            auto_replace_topic_alias_send: false,
            pingresp_recv_timeout_ms: 0,
            connection_establish_timeout_ms: 0,
            shutdown_timeout_ms: 5000,
        }
    }
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting in progress
    Connecting,
    /// Connected and ready
    Connected,
    /// Connection lost, attempting to reconnect
    Reconnecting,
    /// Connection closed permanently
    Closed,
}

// Note: Message type removed - now using mqtt::packet::Packet directly
// Connection events are handled internally via state management
