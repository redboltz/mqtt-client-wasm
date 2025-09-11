//! WASM interface for JavaScript bindings
//!
//! This module provides JavaScript-friendly wrappers around the core MqttClient.
//! Packet constructors accept JSON objects for flexible configuration.
//! Optional fields can be omitted (null/undefined in JavaScript).

use crate::{mqtt, MqttClient, MqttConfig};
use mqtt::packet::{Properties, Property};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

/// Check WASM version for debugging
#[wasm_bindgen]
pub fn check_version() -> String {
    web_sys::console::log_1(
        &"🔥 MANUAL: NEW WASM VERSION LOADED 2026-01-22 FLEXIBLE-PACKETS 🔥".into(),
    );
    "NEW WASM VERSION 2026-01-22 FLEXIBLE-PACKETS".to_string()
}

// ============================================================================
// Packet Option Structs (for JSON deserialization)
// ============================================================================

/// Options for Connect packet (both versions)
/// - `cleanSession` is used for V3.1.1 (maps to clean_session)
/// - `cleanSession` is also used for V5.0 (maps to clean_start)
/// - V5.0 properties are ignored when using V3.1.1 method
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOptions {
    pub client_id: String,
    pub keep_alive: Option<u16>,
    /// Clean session flag (V3.1.1) / Clean start flag (V5.0)
    pub clean_session: Option<bool>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub will_topic: Option<String>,
    pub will_payload: Option<String>,
    pub will_qos: Option<u8>,
    pub will_retain: Option<bool>,
    // V5.0 Properties (ignored for V3.1.1)
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<u16>,
    pub maximum_packet_size: Option<u32>,
    pub topic_alias_maximum: Option<u16>,
    pub request_response_information: Option<bool>,
    pub request_problem_information: Option<bool>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Vec<u8>>,
}

/// User property key-value pair
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPropertyEntry {
    pub key: String,
    pub value: String,
}

/// Options for Publish packet (both versions)
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishOptions {
    pub topic_name: String,
    pub payload: Option<String>,
    pub payload_bytes: Option<Vec<u8>>,
    pub qos: Option<u8>,
    pub retain: Option<bool>,
    pub dup: Option<bool>,
    pub packet_id: Option<u16>,
    // V5.0 properties
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub topic_alias: Option<u16>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<Vec<u8>>,
    pub content_type: Option<String>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

/// Options for Subscribe packet
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeOptions {
    pub packet_id: u16,
    pub subscriptions: Vec<SubscriptionEntry>,
    // V5.0 property
    pub subscription_identifier: Option<u32>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

/// Single subscription entry
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionEntry {
    pub topic: String,
    pub qos: Option<u8>,
    // V5.0 options
    pub no_local: Option<bool>,
    pub retain_as_published: Option<bool>,
    pub retain_handling: Option<u8>,
}

/// Options for Unsubscribe packet
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeOptions {
    pub packet_id: u16,
    pub topics: Vec<String>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

/// Options for MQTT client configuration
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOptions {
    /// MQTT version: "3.1.1", "5.0", "v5.0", "V5_0", etc. Default: "3.1.1"
    pub version: Option<String>,
    /// Ping request send interval in milliseconds. None = disabled
    pub pingreq_send_interval_ms: Option<u32>,
    /// Auto respond to QoS PUBLISH/PUBREC/PUBREL. Default: true
    pub auto_pub_response: Option<bool>,
    /// Auto respond to PINGREQ. Default: true
    pub auto_ping_response: Option<bool>,
    /// Auto map topic alias for send. Default: false
    pub auto_map_topic_alias_send: Option<bool>,
    /// Auto replace topic alias for send. Default: false
    pub auto_replace_topic_alias_send: Option<bool>,
    /// Ping response receive timeout in milliseconds. 0 = disabled
    pub pingresp_recv_timeout_ms: Option<u32>,
    /// Connection establish timeout in milliseconds. 0 = disabled
    pub connection_establish_timeout_ms: Option<u32>,
    /// Shutdown timeout in milliseconds. 0 = disabled
    pub shutdown_timeout_ms: Option<u32>,
}

/// Options for Puback/Pubrec/Pubrel/Pubcomp packets
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PubResponseOptions {
    pub packet_id: u16,
    pub reason_code: Option<u8>,
    pub reason_string: Option<String>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

/// Options for Disconnect packet (both versions)
/// V3.1.1 Disconnect has no fields, so all options are ignored for V3.1.1
/// V5.0 uses all fields
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectOptions {
    pub reason_code: Option<u8>,
    pub reason_string: Option<String>,
    pub session_expiry_interval: Option<u32>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

/// Options for Auth packet (v5.0 only)
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthOptions {
    pub reason_code: Option<u8>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Vec<u8>>,
    pub reason_string: Option<String>,
    pub user_properties: Option<Vec<UserPropertyEntry>>,
}

// ============================================================================
// Helper functions for building Properties
// ============================================================================

fn build_user_properties(
    props: &mut Vec<Property>,
    user_properties: &Option<Vec<UserPropertyEntry>>,
) -> Result<(), JsValue> {
    if let Some(ups) = user_properties {
        for up in ups {
            let prop = mqtt::packet::UserProperty::new(&up.key, &up.value)
                .map_err(|e| JsValue::from_str(&format!("Invalid user property: {:?}", e)))?;
            props.push(Property::UserProperty(prop));
        }
    }
    Ok(())
}

// ============================================================================
// Packet Type Wrapper (for JavaScript)
// ============================================================================

/// Packet type enum exposed to JavaScript
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WasmPacketType {
    Connect,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
    Auth,
}

impl From<mqtt::packet::PacketType> for WasmPacketType {
    fn from(pt: mqtt::packet::PacketType) -> Self {
        match pt {
            mqtt::packet::PacketType::Connect => WasmPacketType::Connect,
            mqtt::packet::PacketType::Connack => WasmPacketType::Connack,
            mqtt::packet::PacketType::Publish => WasmPacketType::Publish,
            mqtt::packet::PacketType::Puback => WasmPacketType::Puback,
            mqtt::packet::PacketType::Pubrec => WasmPacketType::Pubrec,
            mqtt::packet::PacketType::Pubrel => WasmPacketType::Pubrel,
            mqtt::packet::PacketType::Pubcomp => WasmPacketType::Pubcomp,
            mqtt::packet::PacketType::Subscribe => WasmPacketType::Subscribe,
            mqtt::packet::PacketType::Suback => WasmPacketType::Suback,
            mqtt::packet::PacketType::Unsubscribe => WasmPacketType::Unsubscribe,
            mqtt::packet::PacketType::Unsuback => WasmPacketType::Unsuback,
            mqtt::packet::PacketType::Pingreq => WasmPacketType::Pingreq,
            mqtt::packet::PacketType::Pingresp => WasmPacketType::Pingresp,
            mqtt::packet::PacketType::Disconnect => WasmPacketType::Disconnect,
            mqtt::packet::PacketType::Auth => WasmPacketType::Auth,
        }
    }
}

// ============================================================================
// PropertiesExt Trait for MQTT v5.0 Properties
// ============================================================================

/// Extension trait for accessing MQTT v5.0 properties
pub trait PropertiesExt {
    fn payload_format_indicator(&self) -> Option<u8>;
    fn message_expiry_interval(&self) -> Option<u32>;
    fn topic_alias(&self) -> Option<u16>;
    fn response_topic(&self) -> Option<String>;
    fn correlation_data(&self) -> Option<Vec<u8>>;
    fn content_type(&self) -> Option<String>;
    fn session_expiry_interval(&self) -> Option<u32>;
    fn receive_maximum(&self) -> Option<u16>;
    fn maximum_qos(&self) -> Option<u8>;
    fn retain_available(&self) -> Option<bool>;
    fn maximum_packet_size(&self) -> Option<u32>;
    fn assigned_client_identifier(&self) -> Option<String>;
    fn topic_alias_maximum(&self) -> Option<u16>;
    fn reason_string(&self) -> Option<String>;
    fn wildcard_subscription_available(&self) -> Option<bool>;
    fn subscription_identifier_available(&self) -> Option<bool>;
    fn shared_subscription_available(&self) -> Option<bool>;
    fn server_keep_alive(&self) -> Option<u16>;
    fn response_information(&self) -> Option<String>;
    fn server_reference(&self) -> Option<String>;
    fn authentication_method(&self) -> Option<String>;
    fn authentication_data(&self) -> Option<Vec<u8>>;
}

impl PropertiesExt for Properties {
    fn payload_format_indicator(&self) -> Option<u8> {
        for prop in self.iter() {
            if let Property::PayloadFormatIndicator(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn message_expiry_interval(&self) -> Option<u32> {
        for prop in self.iter() {
            if let Property::MessageExpiryInterval(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn topic_alias(&self) -> Option<u16> {
        for prop in self.iter() {
            if let Property::TopicAlias(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn response_topic(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::ResponseTopic(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn correlation_data(&self) -> Option<Vec<u8>> {
        for prop in self.iter() {
            if let Property::CorrelationData(p) = prop {
                return Some(p.val().to_vec());
            }
        }
        None
    }

    fn content_type(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::ContentType(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn session_expiry_interval(&self) -> Option<u32> {
        for prop in self.iter() {
            if let Property::SessionExpiryInterval(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn receive_maximum(&self) -> Option<u16> {
        for prop in self.iter() {
            if let Property::ReceiveMaximum(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn maximum_qos(&self) -> Option<u8> {
        for prop in self.iter() {
            if let Property::MaximumQos(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn retain_available(&self) -> Option<bool> {
        for prop in self.iter() {
            if let Property::RetainAvailable(p) = prop {
                return Some(p.val() != 0);
            }
        }
        None
    }

    fn maximum_packet_size(&self) -> Option<u32> {
        for prop in self.iter() {
            if let Property::MaximumPacketSize(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn assigned_client_identifier(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::AssignedClientIdentifier(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn topic_alias_maximum(&self) -> Option<u16> {
        for prop in self.iter() {
            if let Property::TopicAliasMaximum(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn reason_string(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::ReasonString(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn wildcard_subscription_available(&self) -> Option<bool> {
        for prop in self.iter() {
            if let Property::WildcardSubscriptionAvailable(p) = prop {
                return Some(p.val() != 0);
            }
        }
        None
    }

    fn subscription_identifier_available(&self) -> Option<bool> {
        for prop in self.iter() {
            if let Property::SubscriptionIdentifierAvailable(p) = prop {
                return Some(p.val() != 0);
            }
        }
        None
    }

    fn shared_subscription_available(&self) -> Option<bool> {
        for prop in self.iter() {
            if let Property::SharedSubscriptionAvailable(p) = prop {
                return Some(p.val() != 0);
            }
        }
        None
    }

    fn server_keep_alive(&self) -> Option<u16> {
        for prop in self.iter() {
            if let Property::ServerKeepAlive(p) = prop {
                return Some(p.val());
            }
        }
        None
    }

    fn response_information(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::ResponseInformation(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn server_reference(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::ServerReference(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn authentication_method(&self) -> Option<String> {
        for prop in self.iter() {
            if let Property::AuthenticationMethod(p) = prop {
                return Some(p.val().to_string());
            }
        }
        None
    }

    fn authentication_data(&self) -> Option<Vec<u8>> {
        for prop in self.iter() {
            if let Property::AuthenticationData(p) = prop {
                return Some(p.val().to_vec());
            }
        }
        None
    }
}

impl PropertiesExt for Option<Properties> {
    fn payload_format_indicator(&self) -> Option<u8> {
        self.as_ref().and_then(|p| p.payload_format_indicator())
    }

    fn message_expiry_interval(&self) -> Option<u32> {
        self.as_ref().and_then(|p| p.message_expiry_interval())
    }

    fn topic_alias(&self) -> Option<u16> {
        self.as_ref().and_then(|p| p.topic_alias())
    }

    fn response_topic(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.response_topic())
    }

    fn correlation_data(&self) -> Option<Vec<u8>> {
        self.as_ref().and_then(|p| p.correlation_data())
    }

    fn content_type(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.content_type())
    }

    fn session_expiry_interval(&self) -> Option<u32> {
        self.as_ref().and_then(|p| p.session_expiry_interval())
    }

    fn receive_maximum(&self) -> Option<u16> {
        self.as_ref().and_then(|p| p.receive_maximum())
    }

    fn maximum_qos(&self) -> Option<u8> {
        self.as_ref().and_then(|p| p.maximum_qos())
    }

    fn retain_available(&self) -> Option<bool> {
        self.as_ref().and_then(|p| p.retain_available())
    }

    fn maximum_packet_size(&self) -> Option<u32> {
        self.as_ref().and_then(|p| p.maximum_packet_size())
    }

    fn assigned_client_identifier(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.assigned_client_identifier())
    }

    fn topic_alias_maximum(&self) -> Option<u16> {
        self.as_ref().and_then(|p| p.topic_alias_maximum())
    }

    fn reason_string(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.reason_string())
    }

    fn wildcard_subscription_available(&self) -> Option<bool> {
        self.as_ref()
            .and_then(|p| p.wildcard_subscription_available())
    }

    fn subscription_identifier_available(&self) -> Option<bool> {
        self.as_ref()
            .and_then(|p| p.subscription_identifier_available())
    }

    fn shared_subscription_available(&self) -> Option<bool> {
        self.as_ref()
            .and_then(|p| p.shared_subscription_available())
    }

    fn server_keep_alive(&self) -> Option<u16> {
        self.as_ref().and_then(|p| p.server_keep_alive())
    }

    fn response_information(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.response_information())
    }

    fn server_reference(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.server_reference())
    }

    fn authentication_method(&self) -> Option<String> {
        self.as_ref().and_then(|p| p.authentication_method())
    }

    fn authentication_data(&self) -> Option<Vec<u8>> {
        self.as_ref().and_then(|p| p.authentication_data())
    }
}

// ============================================================================
// Version-Specific Packet Wrappers
// ============================================================================

// ----------------------------------------------------------------------------
// V3.1.1 Packet Wrappers
// ----------------------------------------------------------------------------

/// WASM wrapper for V3.1.1 PUBLISH packet
#[wasm_bindgen]
pub struct WasmPublishPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Publish,
}

#[wasm_bindgen]
impl WasmPublishPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = topicName)]
    pub fn topic_name(&self) -> String {
        self.inner.topic_name().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn payload(&self) -> Option<String> {
        std::str::from_utf8(self.inner.payload().as_slice())
            .ok()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(js_name = payloadBytes)]
    pub fn payload_bytes(&self) -> Vec<u8> {
        self.inner.payload().as_slice().to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn qos(&self) -> u8 {
        self.inner.qos() as u8
    }

    #[wasm_bindgen(getter)]
    pub fn retain(&self) -> bool {
        self.inner.retain()
    }

    #[wasm_bindgen(getter)]
    pub fn dup(&self) -> bool {
        self.inner.dup()
    }

    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> Option<u16> {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 CONNACK packet
#[wasm_bindgen]
pub struct WasmConnackPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Connack,
}

#[wasm_bindgen]
impl WasmConnackPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = sessionPresent)]
    pub fn session_present(&self) -> bool {
        self.inner.session_present()
    }

    #[wasm_bindgen(getter, js_name = returnCode)]
    pub fn return_code(&self) -> u8 {
        self.inner.return_code() as u8
    }

    #[wasm_bindgen(js_name = isSuccess)]
    pub fn is_success(&self) -> bool {
        self.inner.return_code() == mqtt::result_code::ConnectReturnCode::Accepted
    }
}

/// WASM wrapper for V3.1.1 SUBACK packet
#[wasm_bindgen]
pub struct WasmSubackPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Suback,
}

#[wasm_bindgen]
impl WasmSubackPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(js_name = returnCodes)]
    pub fn return_codes(&self) -> Vec<u8> {
        self.inner.return_codes().iter().map(|c| *c as u8).collect()
    }
}

/// WASM wrapper for V3.1.1 UNSUBACK packet
#[wasm_bindgen]
pub struct WasmUnsubackPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Unsuback,
}

#[wasm_bindgen]
impl WasmUnsubackPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 PUBACK packet
#[wasm_bindgen]
pub struct WasmPubackPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Puback,
}

#[wasm_bindgen]
impl WasmPubackPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 PUBREC packet
#[wasm_bindgen]
pub struct WasmPubrecPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Pubrec,
}

#[wasm_bindgen]
impl WasmPubrecPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 PUBREL packet
#[wasm_bindgen]
pub struct WasmPubrelPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Pubrel,
}

#[wasm_bindgen]
impl WasmPubrelPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 PUBCOMP packet
#[wasm_bindgen]
pub struct WasmPubcompPacketV3_1_1 {
    inner: mqtt::packet::v3_1_1::Pubcomp,
}

#[wasm_bindgen]
impl WasmPubcompPacketV3_1_1 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }
}

/// WASM wrapper for V3.1.1 DISCONNECT packet
#[wasm_bindgen]
pub struct WasmDisconnectPacketV3_1_1 {
    #[allow(dead_code)]
    inner: mqtt::packet::v3_1_1::Disconnect,
}

#[wasm_bindgen]
impl WasmDisconnectPacketV3_1_1 {
    // V3.1.1 Disconnect has no fields
}

// ----------------------------------------------------------------------------
// V5.0 Packet Wrappers
// ----------------------------------------------------------------------------

/// WASM wrapper for V5.0 PUBLISH packet
#[wasm_bindgen]
pub struct WasmPublishPacketV5_0 {
    inner: mqtt::packet::v5_0::Publish,
}

#[wasm_bindgen]
impl WasmPublishPacketV5_0 {
    #[wasm_bindgen(getter, js_name = topicName)]
    pub fn topic_name(&self) -> String {
        self.inner.topic_name().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn payload(&self) -> Option<String> {
        std::str::from_utf8(self.inner.payload().as_slice())
            .ok()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(js_name = payloadBytes)]
    pub fn payload_bytes(&self) -> Vec<u8> {
        self.inner.payload().as_slice().to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn qos(&self) -> u8 {
        self.inner.qos() as u8
    }

    #[wasm_bindgen(getter)]
    pub fn retain(&self) -> bool {
        self.inner.retain()
    }

    #[wasm_bindgen(getter)]
    pub fn dup(&self) -> bool {
        self.inner.dup()
    }

    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> Option<u16> {
        self.inner.packet_id()
    }

    // V5.0 Properties
    #[wasm_bindgen(getter, js_name = payloadFormatIndicator)]
    pub fn payload_format_indicator(&self) -> Option<u8> {
        self.inner.props.payload_format_indicator()
    }

    #[wasm_bindgen(getter, js_name = messageExpiryInterval)]
    pub fn message_expiry_interval(&self) -> Option<u32> {
        self.inner.props.message_expiry_interval()
    }

    #[wasm_bindgen(getter, js_name = topicAlias)]
    pub fn topic_alias(&self) -> Option<u16> {
        self.inner.props.topic_alias()
    }

    #[wasm_bindgen(getter, js_name = responseTopic)]
    pub fn response_topic(&self) -> Option<String> {
        self.inner.props.response_topic()
    }

    #[wasm_bindgen(getter, js_name = correlationData)]
    pub fn correlation_data(&self) -> Option<Vec<u8>> {
        self.inner.props.correlation_data()
    }

    #[wasm_bindgen(getter, js_name = contentType)]
    pub fn content_type(&self) -> Option<String> {
        self.inner.props.content_type()
    }
}

/// WASM wrapper for V5.0 CONNACK packet
#[wasm_bindgen]
pub struct WasmConnackPacketV5_0 {
    inner: mqtt::packet::v5_0::Connack,
}

#[wasm_bindgen]
impl WasmConnackPacketV5_0 {
    #[wasm_bindgen(getter, js_name = sessionPresent)]
    pub fn session_present(&self) -> bool {
        self.inner.session_present()
    }

    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code() as u8
    }

    #[wasm_bindgen(js_name = isSuccess)]
    pub fn is_success(&self) -> bool {
        self.inner.reason_code() == mqtt::result_code::ConnectReasonCode::Success
    }

    // V5.0 Properties
    #[wasm_bindgen(getter, js_name = sessionExpiryInterval)]
    pub fn session_expiry_interval(&self) -> Option<u32> {
        self.inner.props.session_expiry_interval()
    }

    #[wasm_bindgen(getter, js_name = receiveMaximum)]
    pub fn receive_maximum(&self) -> Option<u16> {
        self.inner.props.receive_maximum()
    }

    #[wasm_bindgen(getter, js_name = maximumQos)]
    pub fn maximum_qos(&self) -> Option<u8> {
        self.inner.props.maximum_qos().map(|q| q as u8)
    }

    #[wasm_bindgen(getter, js_name = retainAvailable)]
    pub fn retain_available(&self) -> Option<bool> {
        self.inner.props.retain_available()
    }

    #[wasm_bindgen(getter, js_name = maximumPacketSize)]
    pub fn maximum_packet_size(&self) -> Option<u32> {
        self.inner.props.maximum_packet_size()
    }

    #[wasm_bindgen(getter, js_name = assignedClientIdentifier)]
    pub fn assigned_client_identifier(&self) -> Option<String> {
        self.inner
            .props
            .assigned_client_identifier()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = topicAliasMaximum)]
    pub fn topic_alias_maximum(&self) -> Option<u16> {
        self.inner.props.topic_alias_maximum()
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = wildcardSubscriptionAvailable)]
    pub fn wildcard_subscription_available(&self) -> Option<bool> {
        self.inner.props.wildcard_subscription_available()
    }

    #[wasm_bindgen(getter, js_name = subscriptionIdentifiersAvailable)]
    pub fn subscription_identifiers_available(&self) -> Option<bool> {
        self.inner.props.subscription_identifier_available()
    }

    #[wasm_bindgen(getter, js_name = sharedSubscriptionAvailable)]
    pub fn shared_subscription_available(&self) -> Option<bool> {
        self.inner.props.shared_subscription_available()
    }

    #[wasm_bindgen(getter, js_name = serverKeepAlive)]
    pub fn server_keep_alive(&self) -> Option<u16> {
        self.inner.props.server_keep_alive()
    }

    #[wasm_bindgen(getter, js_name = responseInformation)]
    pub fn response_information(&self) -> Option<String> {
        self.inner
            .props
            .response_information()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = serverReference)]
    pub fn server_reference(&self) -> Option<String> {
        self.inner.props.server_reference().map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = authenticationMethod)]
    pub fn authentication_method(&self) -> Option<String> {
        self.inner
            .props
            .authentication_method()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = authenticationData)]
    pub fn authentication_data(&self) -> Option<Vec<u8>> {
        self.inner.props.authentication_data().map(|s| s.to_vec())
    }
}

/// WASM wrapper for V5.0 SUBACK packet
#[wasm_bindgen]
pub struct WasmSubackPacketV5_0 {
    inner: mqtt::packet::v5_0::Suback,
}

#[wasm_bindgen]
impl WasmSubackPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(js_name = reasonCodes)]
    pub fn reason_codes(&self) -> Vec<u8> {
        self.inner.reason_codes().iter().map(|c| *c as u8).collect()
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 UNSUBACK packet
#[wasm_bindgen]
pub struct WasmUnsubackPacketV5_0 {
    inner: mqtt::packet::v5_0::Unsuback,
}

#[wasm_bindgen]
impl WasmUnsubackPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(js_name = reasonCodes)]
    pub fn reason_codes(&self) -> Vec<u8> {
        self.inner.reason_codes().iter().map(|c| *c as u8).collect()
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 PUBACK packet
#[wasm_bindgen]
pub struct WasmPubackPacketV5_0 {
    inner: mqtt::packet::v5_0::Puback,
}

#[wasm_bindgen]
impl WasmPubackPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 PUBREC packet
#[wasm_bindgen]
pub struct WasmPubrecPacketV5_0 {
    inner: mqtt::packet::v5_0::Pubrec,
}

#[wasm_bindgen]
impl WasmPubrecPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 PUBREL packet
#[wasm_bindgen]
pub struct WasmPubrelPacketV5_0 {
    inner: mqtt::packet::v5_0::Pubrel,
}

#[wasm_bindgen]
impl WasmPubrelPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 PUBCOMP packet
#[wasm_bindgen]
pub struct WasmPubcompPacketV5_0 {
    inner: mqtt::packet::v5_0::Pubcomp,
}

#[wasm_bindgen]
impl WasmPubcompPacketV5_0 {
    #[wasm_bindgen(getter, js_name = packetId)]
    pub fn packet_id(&self) -> u16 {
        self.inner.packet_id()
    }

    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 DISCONNECT packet
#[wasm_bindgen]
pub struct WasmDisconnectPacketV5_0 {
    inner: mqtt::packet::v5_0::Disconnect,
}

#[wasm_bindgen]
impl WasmDisconnectPacketV5_0 {
    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = sessionExpiryInterval)]
    pub fn session_expiry_interval(&self) -> Option<u32> {
        self.inner.props.session_expiry_interval()
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = serverReference)]
    pub fn server_reference(&self) -> Option<String> {
        self.inner.props.server_reference().map(|s| s.to_string())
    }
}

/// WASM wrapper for V5.0 AUTH packet
#[wasm_bindgen]
pub struct WasmAuthPacketV5_0 {
    inner: mqtt::packet::v5_0::Auth,
}

#[wasm_bindgen]
impl WasmAuthPacketV5_0 {
    #[wasm_bindgen(getter, js_name = reasonCode)]
    pub fn reason_code(&self) -> u8 {
        self.inner.reason_code().map_or(0, |c| c as u8)
    }

    #[wasm_bindgen(getter, js_name = authenticationMethod)]
    pub fn authentication_method(&self) -> Option<String> {
        self.inner
            .props
            .authentication_method()
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(getter, js_name = authenticationData)]
    pub fn authentication_data(&self) -> Option<Vec<u8>> {
        self.inner.props.authentication_data().map(|s| s.to_vec())
    }

    #[wasm_bindgen(getter, js_name = reasonString)]
    pub fn reason_string(&self) -> Option<String> {
        self.inner.props.reason_string().map(|s| s.to_string())
    }
}

// ============================================================================
// WASM Packet Wrapper
// ============================================================================

/// WASM wrapper for MQTT packets
#[wasm_bindgen]
pub struct WasmMqttPacket {
    inner: mqtt::packet::Packet,
}

#[wasm_bindgen]
impl WasmMqttPacket {
    /// Get packet type as enum
    #[wasm_bindgen(js_name = packetType)]
    pub fn packet_type(&self) -> WasmPacketType {
        WasmPacketType::from(self.inner.packet_type())
    }

    /// Get packet type as string (for debugging)
    #[wasm_bindgen(js_name = packetTypeString)]
    pub fn packet_type_string(&self) -> String {
        format!("{:?}", self.inner.packet_type())
    }

    // ------------------------------------------------------------------------
    // V3.1.1 Packet Constructors
    // ------------------------------------------------------------------------

    /// Create V3.1.1 Connect packet from JSON options
    #[wasm_bindgen(js_name = newConnectV311)]
    pub fn new_connect_v311(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: ConnectOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v3_1_1::Connect::builder()
            .client_id(&opts.client_id)
            .map_err(|e| JsValue::from_str(&format!("Invalid client ID: {:?}", e)))?;

        if let Some(keep_alive) = opts.keep_alive {
            builder = builder.keep_alive(keep_alive);
        }
        if let Some(clean_session) = opts.clean_session {
            builder = builder.clean_session(clean_session);
        }
        if let Some(ref user_name) = opts.user_name {
            builder = builder
                .user_name(user_name)
                .map_err(|e| JsValue::from_str(&format!("Invalid user name: {:?}", e)))?;
        }
        if let Some(ref password) = opts.password {
            builder = builder
                .password(password.as_bytes().to_vec())
                .map_err(|e| JsValue::from_str(&format!("Invalid password: {:?}", e)))?;
        }

        // Will message
        if let Some(ref will_topic) = opts.will_topic {
            let will_payload = opts.will_payload.as_deref().unwrap_or("");
            let will_qos = mqtt::packet::Qos::try_from(opts.will_qos.unwrap_or(0))
                .map_err(|e| JsValue::from_str(&format!("Invalid will QoS: {:?}", e)))?;
            let will_retain = opts.will_retain.unwrap_or(false);

            builder = builder
                .will_message(
                    will_topic,
                    will_payload.as_bytes().to_vec(),
                    will_qos,
                    will_retain,
                )
                .map_err(|e| JsValue::from_str(&format!("Invalid will message: {:?}", e)))?;
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build CONNECT: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Connect(packet),
        })
    }

    /// Create V3.1.1 Publish packet from JSON options
    #[wasm_bindgen(js_name = newPublishV311)]
    pub fn new_publish_v311(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PublishOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let qos = mqtt::packet::Qos::try_from(opts.qos.unwrap_or(0))
            .map_err(|e| JsValue::from_str(&format!("Invalid QoS: {:?}", e)))?;

        let mut builder = mqtt::packet::v3_1_1::Publish::builder()
            .topic_name(&opts.topic_name)
            .map_err(|e| JsValue::from_str(&format!("Invalid topic: {:?}", e)))?
            .qos(qos);

        // Payload (string or bytes)
        if let Some(ref payload_bytes) = opts.payload_bytes {
            builder = builder.payload(payload_bytes.clone());
        } else if let Some(ref payload) = opts.payload {
            builder = builder.payload(payload.as_bytes().to_vec());
        }

        if let Some(retain) = opts.retain {
            builder = builder.retain(retain);
        }
        if let Some(dup) = opts.dup {
            builder = builder.dup(dup);
        }
        if let Some(packet_id) = opts.packet_id {
            builder = builder.packet_id(packet_id);
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBLISH: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Publish(packet),
        })
    }

    /// Create V3.1.1 Subscribe packet from JSON options
    #[wasm_bindgen(js_name = newSubscribeV311)]
    pub fn new_subscribe_v311(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: SubscribeOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let entries: Result<Vec<_>, _> = opts
            .subscriptions
            .iter()
            .map(|sub| {
                let qos = mqtt::packet::Qos::try_from(sub.qos.unwrap_or(0))
                    .map_err(|e| JsValue::from_str(&format!("Invalid QoS: {:?}", e)))?;
                let sub_opts = mqtt::packet::SubOpts::new().set_qos(qos);
                mqtt::packet::SubEntry::new(&sub.topic, sub_opts)
                    .map_err(|e| JsValue::from_str(&format!("Invalid topic filter: {:?}", e)))
            })
            .collect();

        let packet = mqtt::packet::v3_1_1::Subscribe::builder()
            .packet_id(opts.packet_id)
            .entries(entries?)
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build SUBSCRIBE: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Subscribe(packet),
        })
    }

    /// Create V3.1.1 Unsubscribe packet from JSON options
    #[wasm_bindgen(js_name = newUnsubscribeV311)]
    pub fn new_unsubscribe_v311(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: UnsubscribeOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        // Unsubscribe entries accepts Vec<&str> or Vec<String>
        let topics: Vec<&str> = opts.topics.iter().map(|s| s.as_str()).collect();

        let packet = mqtt::packet::v3_1_1::Unsubscribe::builder()
            .packet_id(opts.packet_id)
            .entries(topics)
            .map_err(|e| JsValue::from_str(&format!("Invalid topic entries: {:?}", e)))?
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build UNSUBSCRIBE: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Unsubscribe(packet),
        })
    }

    /// Create V3.1.1 Puback packet
    #[wasm_bindgen(js_name = newPubackV311)]
    pub fn new_puback_v311(packet_id: u16) -> Result<WasmMqttPacket, JsValue> {
        let packet = mqtt::packet::v3_1_1::Puback::builder()
            .packet_id(packet_id)
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBACK: {:?}", e)))?;
        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Puback(packet),
        })
    }

    /// Create V3.1.1 Pubrec packet
    #[wasm_bindgen(js_name = newPubrecV311)]
    pub fn new_pubrec_v311(packet_id: u16) -> Result<WasmMqttPacket, JsValue> {
        let packet = mqtt::packet::v3_1_1::Pubrec::builder()
            .packet_id(packet_id)
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBREC: {:?}", e)))?;
        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Pubrec(packet),
        })
    }

    /// Create V3.1.1 Pubrel packet
    #[wasm_bindgen(js_name = newPubrelV311)]
    pub fn new_pubrel_v311(packet_id: u16) -> Result<WasmMqttPacket, JsValue> {
        let packet = mqtt::packet::v3_1_1::Pubrel::builder()
            .packet_id(packet_id)
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBREL: {:?}", e)))?;
        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Pubrel(packet),
        })
    }

    /// Create V3.1.1 Pubcomp packet
    #[wasm_bindgen(js_name = newPubcompV311)]
    pub fn new_pubcomp_v311(packet_id: u16) -> Result<WasmMqttPacket, JsValue> {
        let packet = mqtt::packet::v3_1_1::Pubcomp::builder()
            .packet_id(packet_id)
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBCOMP: {:?}", e)))?;
        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Pubcomp(packet),
        })
    }

    /// Create V3.1.1 Pingreq packet
    #[wasm_bindgen(js_name = newPingreqV311)]
    pub fn new_pingreq_v311() -> WasmMqttPacket {
        let packet = mqtt::packet::v3_1_1::Pingreq::new();
        WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Pingreq(packet),
        }
    }

    /// Create V3.1.1 Disconnect packet
    #[wasm_bindgen(js_name = newDisconnectV311)]
    pub fn new_disconnect_v311() -> WasmMqttPacket {
        let packet = mqtt::packet::v3_1_1::Disconnect::new();
        WasmMqttPacket {
            inner: mqtt::packet::Packet::V3_1_1Disconnect(packet),
        }
    }

    // ------------------------------------------------------------------------
    // V5.0 Packet Constructors
    // ------------------------------------------------------------------------

    /// Create V5.0 Connect packet from JSON options
    #[wasm_bindgen(js_name = newConnectV50)]
    pub fn new_connect_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: ConnectOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Connect::builder()
            .client_id(&opts.client_id)
            .map_err(|e| JsValue::from_str(&format!("Invalid client ID: {:?}", e)))?;

        if let Some(keep_alive) = opts.keep_alive {
            builder = builder.keep_alive(keep_alive);
        }
        // clean_session maps to clean_start in V5.0
        if let Some(clean_session) = opts.clean_session {
            builder = builder.clean_start(clean_session);
        }
        if let Some(ref user_name) = opts.user_name {
            builder = builder
                .user_name(user_name)
                .map_err(|e| JsValue::from_str(&format!("Invalid user name: {:?}", e)))?;
        }
        if let Some(ref password) = opts.password {
            builder = builder
                .password(password.as_bytes().to_vec())
                .map_err(|e| JsValue::from_str(&format!("Invalid password: {:?}", e)))?;
        }

        // Will message
        if let Some(ref will_topic) = opts.will_topic {
            let will_payload = opts.will_payload.as_deref().unwrap_or("");
            let will_qos = mqtt::packet::Qos::try_from(opts.will_qos.unwrap_or(0))
                .map_err(|e| JsValue::from_str(&format!("Invalid will QoS: {:?}", e)))?;
            let will_retain = opts.will_retain.unwrap_or(false);

            builder = builder
                .will_message(
                    will_topic,
                    will_payload.as_bytes().to_vec(),
                    will_qos,
                    will_retain,
                )
                .map_err(|e| JsValue::from_str(&format!("Invalid will message: {:?}", e)))?;
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();

        if let Some(interval) = opts.session_expiry_interval {
            let prop = mqtt::packet::SessionExpiryInterval::new(interval).map_err(|e| {
                JsValue::from_str(&format!("Invalid session expiry interval: {:?}", e))
            })?;
            props_vec.push(Property::SessionExpiryInterval(prop));
        }
        if let Some(max) = opts.receive_maximum {
            let prop = mqtt::packet::ReceiveMaximum::new(max)
                .map_err(|e| JsValue::from_str(&format!("Invalid receive maximum: {:?}", e)))?;
            props_vec.push(Property::ReceiveMaximum(prop));
        }
        if let Some(size) = opts.maximum_packet_size {
            let prop = mqtt::packet::MaximumPacketSize::new(size)
                .map_err(|e| JsValue::from_str(&format!("Invalid maximum packet size: {:?}", e)))?;
            props_vec.push(Property::MaximumPacketSize(prop));
        }
        if let Some(max) = opts.topic_alias_maximum {
            let prop = mqtt::packet::TopicAliasMaximum::new(max)
                .map_err(|e| JsValue::from_str(&format!("Invalid topic alias maximum: {:?}", e)))?;
            props_vec.push(Property::TopicAliasMaximum(prop));
        }
        if let Some(v) = opts.request_response_information {
            let prop = mqtt::packet::RequestResponseInformation::new(v as u8).map_err(|e| {
                JsValue::from_str(&format!("Invalid request response information: {:?}", e))
            })?;
            props_vec.push(Property::RequestResponseInformation(prop));
        }
        if let Some(v) = opts.request_problem_information {
            let prop = mqtt::packet::RequestProblemInformation::new(v as u8).map_err(|e| {
                JsValue::from_str(&format!("Invalid request problem information: {:?}", e))
            })?;
            props_vec.push(Property::RequestProblemInformation(prop));
        }
        if let Some(ref method) = opts.authentication_method {
            let prop = mqtt::packet::AuthenticationMethod::new(method).map_err(|e| {
                JsValue::from_str(&format!("Invalid authentication method: {:?}", e))
            })?;
            props_vec.push(Property::AuthenticationMethod(prop));
        }
        if let Some(ref data) = opts.authentication_data {
            let prop = mqtt::packet::AuthenticationData::new(data.clone())
                .map_err(|e| JsValue::from_str(&format!("Invalid authentication data: {:?}", e)))?;
            props_vec.push(Property::AuthenticationData(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build CONNECT: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Connect(packet),
        })
    }

    /// Create V5.0 Publish packet from JSON options
    #[wasm_bindgen(js_name = newPublishV50)]
    pub fn new_publish_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PublishOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let qos = mqtt::packet::Qos::try_from(opts.qos.unwrap_or(0))
            .map_err(|e| JsValue::from_str(&format!("Invalid QoS: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Publish::builder()
            .topic_name(&opts.topic_name)
            .map_err(|e| JsValue::from_str(&format!("Invalid topic: {:?}", e)))?
            .qos(qos);

        // Payload
        if let Some(ref payload_bytes) = opts.payload_bytes {
            builder = builder.payload(payload_bytes.clone());
        } else if let Some(ref payload) = opts.payload {
            builder = builder.payload(payload.as_bytes().to_vec());
        }

        if let Some(retain) = opts.retain {
            builder = builder.retain(retain);
        }
        if let Some(dup) = opts.dup {
            builder = builder.dup(dup);
        }
        if let Some(packet_id) = opts.packet_id {
            builder = builder.packet_id(packet_id);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();

        if let Some(indicator) = opts.payload_format_indicator {
            let format = mqtt::packet::PayloadFormat::try_from(indicator)
                .map_err(|e| JsValue::from_str(&format!("Invalid payload format: {:?}", e)))?;
            let prop = mqtt::packet::PayloadFormatIndicator::new(format).map_err(|e| {
                JsValue::from_str(&format!("Invalid payload format indicator: {:?}", e))
            })?;
            props_vec.push(Property::PayloadFormatIndicator(prop));
        }
        if let Some(interval) = opts.message_expiry_interval {
            let prop = mqtt::packet::MessageExpiryInterval::new(interval).map_err(|e| {
                JsValue::from_str(&format!("Invalid message expiry interval: {:?}", e))
            })?;
            props_vec.push(Property::MessageExpiryInterval(prop));
        }
        if let Some(alias) = opts.topic_alias {
            let prop = mqtt::packet::TopicAlias::new(alias)
                .map_err(|e| JsValue::from_str(&format!("Invalid topic alias: {:?}", e)))?;
            props_vec.push(Property::TopicAlias(prop));
        }
        if let Some(ref topic) = opts.response_topic {
            let prop = mqtt::packet::ResponseTopic::new(topic)
                .map_err(|e| JsValue::from_str(&format!("Invalid response topic: {:?}", e)))?;
            props_vec.push(Property::ResponseTopic(prop));
        }
        if let Some(ref data) = opts.correlation_data {
            let prop = mqtt::packet::CorrelationData::new(data.clone())
                .map_err(|e| JsValue::from_str(&format!("Invalid correlation data: {:?}", e)))?;
            props_vec.push(Property::CorrelationData(prop));
        }
        if let Some(ref content_type) = opts.content_type {
            let prop = mqtt::packet::ContentType::new(content_type)
                .map_err(|e| JsValue::from_str(&format!("Invalid content type: {:?}", e)))?;
            props_vec.push(Property::ContentType(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBLISH: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Publish(packet),
        })
    }

    /// Create V5.0 Subscribe packet from JSON options
    #[wasm_bindgen(js_name = newSubscribeV50)]
    pub fn new_subscribe_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: SubscribeOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let entries: Result<Vec<_>, _> = opts
            .subscriptions
            .iter()
            .map(|sub| {
                let qos = mqtt::packet::Qos::try_from(sub.qos.unwrap_or(0))
                    .map_err(|e| JsValue::from_str(&format!("Invalid QoS: {:?}", e)))?;

                let mut sub_opts = mqtt::packet::SubOpts::new().set_qos(qos);

                if let Some(no_local) = sub.no_local {
                    sub_opts = sub_opts.set_nl(no_local);
                }
                if let Some(rap) = sub.retain_as_published {
                    sub_opts = sub_opts.set_rap(rap);
                }
                if let Some(rh) = sub.retain_handling {
                    let retain_handling =
                        mqtt::packet::RetainHandling::try_from(rh).map_err(|e| {
                            JsValue::from_str(&format!("Invalid retain handling: {:?}", e))
                        })?;
                    sub_opts = sub_opts.set_rh(retain_handling);
                }

                mqtt::packet::SubEntry::new(&sub.topic, sub_opts)
                    .map_err(|e| JsValue::from_str(&format!("Invalid topic filter: {:?}", e)))
            })
            .collect();

        let mut builder = mqtt::packet::v5_0::Subscribe::builder()
            .packet_id(opts.packet_id)
            .entries(entries?);

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();

        if let Some(id) = opts.subscription_identifier {
            let prop = mqtt::packet::SubscriptionIdentifier::new(id).map_err(|e| {
                JsValue::from_str(&format!("Invalid subscription identifier: {:?}", e))
            })?;
            props_vec.push(Property::SubscriptionIdentifier(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build SUBSCRIBE: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Subscribe(packet),
        })
    }

    /// Create V5.0 Unsubscribe packet from JSON options
    #[wasm_bindgen(js_name = newUnsubscribeV50)]
    pub fn new_unsubscribe_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: UnsubscribeOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let topics: Vec<&str> = opts.topics.iter().map(|s| s.as_str()).collect();

        let mut builder = mqtt::packet::v5_0::Unsubscribe::builder()
            .packet_id(opts.packet_id)
            .entries(topics)
            .map_err(|e| JsValue::from_str(&format!("Invalid topic entries: {:?}", e)))?;

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build UNSUBSCRIBE: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Unsubscribe(packet),
        })
    }

    /// Create V5.0 Puback packet from JSON options
    #[wasm_bindgen(js_name = newPubackV50)]
    pub fn new_puback_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Puback::builder().packet_id(opts.packet_id);

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::PubackReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBACK: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Puback(packet),
        })
    }

    /// Create V5.0 Pubrec packet from JSON options
    #[wasm_bindgen(js_name = newPubrecV50)]
    pub fn new_pubrec_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Pubrec::builder().packet_id(opts.packet_id);

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::PubrecReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBREC: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Pubrec(packet),
        })
    }

    /// Create V5.0 Pubrel packet from JSON options
    #[wasm_bindgen(js_name = newPubrelV50)]
    pub fn new_pubrel_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Pubrel::builder().packet_id(opts.packet_id);

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::PubrelReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBREL: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Pubrel(packet),
        })
    }

    /// Create V5.0 Pubcomp packet from JSON options
    #[wasm_bindgen(js_name = newPubcompV50)]
    pub fn new_pubcomp_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Pubcomp::builder().packet_id(opts.packet_id);

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::PubcompReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build PUBCOMP: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Pubcomp(packet),
        })
    }

    /// Create V5.0 Pingreq packet
    #[wasm_bindgen(js_name = newPingreqV50)]
    pub fn new_pingreq_v50() -> WasmMqttPacket {
        let packet = mqtt::packet::v5_0::Pingreq::new();
        WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Pingreq(packet),
        }
    }

    /// Create V5.0 Disconnect packet from JSON options
    #[wasm_bindgen(js_name = newDisconnectV50)]
    pub fn new_disconnect_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: DisconnectOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Disconnect::builder();

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::DisconnectReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        if let Some(interval) = opts.session_expiry_interval {
            let prop = mqtt::packet::SessionExpiryInterval::new(interval).map_err(|e| {
                JsValue::from_str(&format!("Invalid session expiry interval: {:?}", e))
            })?;
            props_vec.push(Property::SessionExpiryInterval(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build DISCONNECT: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Disconnect(packet),
        })
    }

    /// Create V5.0 Auth packet from JSON options
    #[wasm_bindgen(js_name = newAuthV50)]
    pub fn new_auth_v50(options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        let opts: AuthOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;

        let mut builder = mqtt::packet::v5_0::Auth::builder();

        if let Some(code) = opts.reason_code {
            let reason_code = mqtt::result_code::AuthReasonCode::try_from(code)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason code: {:?}", e)))?;
            builder = builder.reason_code(reason_code);
        }

        // V5.0 Properties
        let mut props_vec: Vec<Property> = Vec::new();
        if let Some(ref method) = opts.authentication_method {
            let prop = mqtt::packet::AuthenticationMethod::new(method).map_err(|e| {
                JsValue::from_str(&format!("Invalid authentication method: {:?}", e))
            })?;
            props_vec.push(Property::AuthenticationMethod(prop));
        }
        if let Some(ref data) = opts.authentication_data {
            let prop = mqtt::packet::AuthenticationData::new(data.clone())
                .map_err(|e| JsValue::from_str(&format!("Invalid authentication data: {:?}", e)))?;
            props_vec.push(Property::AuthenticationData(prop));
        }
        if let Some(ref reason) = opts.reason_string {
            let prop = mqtt::packet::ReasonString::new(reason)
                .map_err(|e| JsValue::from_str(&format!("Invalid reason string: {:?}", e)))?;
            props_vec.push(Property::ReasonString(prop));
        }
        build_user_properties(&mut props_vec, &opts.user_properties)?;

        if !props_vec.is_empty() {
            builder = builder.props(Properties::from(props_vec));
        }

        let packet = builder
            .build()
            .map_err(|e| JsValue::from_str(&format!("Failed to build AUTH: {:?}", e)))?;

        Ok(WasmMqttPacket {
            inner: mqtt::packet::Packet::V5_0Auth(packet),
        })
    }
}

// ============================================================================
// WASM Config and Client
// ============================================================================

/// WASM-friendly wrapper around MqttConfig
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmMqttConfig {
    inner: MqttConfig,
}

#[wasm_bindgen]
impl WasmMqttConfig {
    /// Create a new MQTT configuration from a JSON object.
    ///
    /// # Example (JavaScript)
    /// ```js
    /// const config = new WasmMqttConfig({
    ///     version: '5.0',
    ///     autoPubResponse: true,
    ///     autoPingResponse: true,
    ///     pingreqSendIntervalMs: 30000,
    /// });
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<WasmMqttConfig, JsValue> {
        let opts: ConfigOptions = if options.is_undefined() || options.is_null() {
            ConfigOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options)
                .map_err(|e| JsValue::from_str(&format!("Invalid config options: {}", e)))?
        };

        let version = match opts.version.as_deref() {
            Some("V5_0") | Some("v5.0") | Some("5.0") | Some("5") => mqtt::Version::V5_0,
            _ => mqtt::Version::V3_1_1, // default
        };

        let config = MqttConfig {
            url: String::new(), // URL is set via connect()
            version,
            pingreq_send_interval_ms: opts.pingreq_send_interval_ms.map(|v| v as u64),
            auto_pub_response: opts.auto_pub_response.unwrap_or(true),
            auto_ping_response: opts.auto_ping_response.unwrap_or(true),
            auto_map_topic_alias_send: opts.auto_map_topic_alias_send.unwrap_or(false),
            auto_replace_topic_alias_send: opts.auto_replace_topic_alias_send.unwrap_or(false),
            pingresp_recv_timeout_ms: opts.pingresp_recv_timeout_ms.map(|v| v as u64).unwrap_or(0),
            connection_establish_timeout_ms: opts
                .connection_establish_timeout_ms
                .map(|v| v as u64)
                .unwrap_or(0),
            shutdown_timeout_ms: opts.shutdown_timeout_ms.map(|v| v as u64).unwrap_or(0),
        };

        Ok(WasmMqttConfig { inner: config })
    }
}

/// WASM-friendly wrapper around MqttClient
#[wasm_bindgen]
pub struct WasmMqttClient {
    inner: MqttClient,
    version: mqtt::Version,
}

#[wasm_bindgen]
impl WasmMqttClient {
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmMqttConfig) -> WasmMqttClient {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &"🔥🔥🔥 NEW WASM FILE LOADED - WASM_INTERFACE: Creating new WasmMqttClient 🔥🔥🔥"
                .into(),
        );
        let version = config.inner.version;
        let client = MqttClient::new(config.inner);
        WasmMqttClient {
            inner: client,
            version,
        }
    }

    /// Connect to MQTT broker
    #[wasm_bindgen]
    pub async fn connect(&self, url: &str) -> std::result::Result<(), JsValue> {
        self.inner
            .connect(url)
            .await
            .map_err(|e| JsValue::from_str(&format!("Connection failed: {:?}", e)))
    }

    /// Get connection state
    #[wasm_bindgen(js_name = isConnected)]
    pub async fn is_connected(&self) -> bool {
        self.inner.is_connected().await
    }

    /// Acquire a packet ID
    #[wasm_bindgen(js_name = acquirePacketId)]
    pub async fn acquire_packet_id(&self) -> Option<u16> {
        self.inner.acquire_packet_id().await
    }

    /// Register a packet ID
    #[wasm_bindgen(js_name = registerPacketId)]
    pub async fn register_packet_id(&self, packet_id: u16) -> bool {
        self.inner.register_packet_id(packet_id).await
    }

    /// Release a packet ID
    #[wasm_bindgen(js_name = releasePacketId)]
    pub async fn release_packet_id(&self, packet_id: u16) -> std::result::Result<(), JsValue> {
        self.inner
            .release_packet_id(packet_id)
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to release packet ID: {:?}", e)))
    }

    /// Send MQTT packet
    #[wasm_bindgen]
    pub async fn send(&self, packet: WasmMqttPacket) -> std::result::Result<(), JsValue> {
        self.inner
            .send(packet.inner)
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to send packet: {:?}", e)))
    }

    /// Receive next packet
    #[wasm_bindgen]
    pub async fn recv(&self) -> std::result::Result<WasmMqttPacket, JsValue> {
        let packet = self
            .inner
            .recv()
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to receive packet: {:?}", e)))?;

        Ok(WasmMqttPacket { inner: packet })
    }

    /// Close the connection
    #[wasm_bindgen]
    pub async fn close(&self) -> std::result::Result<(), JsValue> {
        self.inner
            .close()
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to close connection: {:?}", e)))
    }

    // ------------------------------------------------------------------------
    // Version-Aware Packet Creation Methods
    // ------------------------------------------------------------------------
    // These methods automatically create packets for the correct protocol version
    // based on the client's configured version.

    /// Create Connect packet (version-aware)
    /// Automatically creates V3.1.1 or V5.0 packet based on client version
    #[wasm_bindgen(js_name = newConnectPacket)]
    pub fn new_connect_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => WasmMqttPacket::new_connect_v311(options),
            mqtt::Version::V5_0 => WasmMqttPacket::new_connect_v50(options),
            _ => WasmMqttPacket::new_connect_v311(options), // default
        }
    }

    /// Create Publish packet (version-aware)
    /// Automatically creates V3.1.1 or V5.0 packet based on client version
    #[wasm_bindgen(js_name = newPublishPacket)]
    pub fn new_publish_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => WasmMqttPacket::new_publish_v311(options),
            mqtt::Version::V5_0 => WasmMqttPacket::new_publish_v50(options),
            _ => WasmMqttPacket::new_publish_v311(options),
        }
    }

    /// Create Subscribe packet (version-aware)
    /// Automatically creates V3.1.1 or V5.0 packet based on client version
    #[wasm_bindgen(js_name = newSubscribePacket)]
    pub fn new_subscribe_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => WasmMqttPacket::new_subscribe_v311(options),
            mqtt::Version::V5_0 => WasmMqttPacket::new_subscribe_v50(options),
            _ => WasmMqttPacket::new_subscribe_v311(options),
        }
    }

    /// Create Unsubscribe packet (version-aware)
    /// Automatically creates V3.1.1 or V5.0 packet based on client version
    #[wasm_bindgen(js_name = newUnsubscribePacket)]
    pub fn new_unsubscribe_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => WasmMqttPacket::new_unsubscribe_v311(options),
            mqtt::Version::V5_0 => WasmMqttPacket::new_unsubscribe_v50(options),
            _ => WasmMqttPacket::new_unsubscribe_v311(options),
        }
    }

    /// Create Puback packet (version-aware)
    /// For V3.1.1: only packet_id is used
    /// For V5.0: packet_id, reason_code, reason_string, user_properties are used
    #[wasm_bindgen(js_name = newPubackPacket)]
    pub fn new_puback_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_puback_v311(opts.packet_id)
            }
            mqtt::Version::V5_0 => WasmMqttPacket::new_puback_v50(options),
            _ => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_puback_v311(opts.packet_id)
            }
        }
    }

    /// Create Pubrec packet (version-aware)
    #[wasm_bindgen(js_name = newPubrecPacket)]
    pub fn new_pubrec_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubrec_v311(opts.packet_id)
            }
            mqtt::Version::V5_0 => WasmMqttPacket::new_pubrec_v50(options),
            _ => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubrec_v311(opts.packet_id)
            }
        }
    }

    /// Create Pubrel packet (version-aware)
    #[wasm_bindgen(js_name = newPubrelPacket)]
    pub fn new_pubrel_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubrel_v311(opts.packet_id)
            }
            mqtt::Version::V5_0 => WasmMqttPacket::new_pubrel_v50(options),
            _ => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubrel_v311(opts.packet_id)
            }
        }
    }

    /// Create Pubcomp packet (version-aware)
    #[wasm_bindgen(js_name = newPubcompPacket)]
    pub fn new_pubcomp_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubcomp_v311(opts.packet_id)
            }
            mqtt::Version::V5_0 => WasmMqttPacket::new_pubcomp_v50(options),
            _ => {
                let opts: PubResponseOptions = serde_wasm_bindgen::from_value(options)
                    .map_err(|e| JsValue::from_str(&format!("Invalid options: {:?}", e)))?;
                WasmMqttPacket::new_pubcomp_v311(opts.packet_id)
            }
        }
    }

    /// Create Pingreq packet (version-aware)
    #[wasm_bindgen(js_name = newPingreqPacket)]
    pub fn new_pingreq_packet(&self) -> WasmMqttPacket {
        match self.version {
            mqtt::Version::V3_1_1 => WasmMqttPacket::new_pingreq_v311(),
            mqtt::Version::V5_0 => WasmMqttPacket::new_pingreq_v50(),
            _ => WasmMqttPacket::new_pingreq_v311(),
        }
    }

    /// Create Disconnect packet (version-aware)
    /// For V3.1.1: options are ignored (Disconnect has no fields)
    /// For V5.0: reason_code, reason_string, session_expiry_interval, user_properties are used
    /// Pass undefined/null for default options
    #[wasm_bindgen(js_name = newDisconnectPacket)]
    pub fn new_disconnect_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V3_1_1 => Ok(WasmMqttPacket::new_disconnect_v311()),
            mqtt::Version::V5_0 => {
                let opts = if options.is_undefined() || options.is_null() {
                    js_sys::Object::new().into()
                } else {
                    options
                };
                WasmMqttPacket::new_disconnect_v50(opts)
            }
            _ => Ok(WasmMqttPacket::new_disconnect_v311()),
        }
    }

    /// Create Auth packet (V5.0 only)
    /// Returns error if called on V3.1.1 client
    #[wasm_bindgen(js_name = newAuthPacket)]
    pub fn new_auth_packet(&self, options: JsValue) -> Result<WasmMqttPacket, JsValue> {
        match self.version {
            mqtt::Version::V5_0 => WasmMqttPacket::new_auth_v50(options),
            _ => Err(JsValue::from_str(
                "AUTH packet is only available in MQTT v5.0",
            )),
        }
    }

    // ------------------------------------------------------------------------
    // Packet Conversion Methods (version-aware)
    // ------------------------------------------------------------------------

    /// Convert packet to PUBLISH wrapper (version-aware)
    /// Returns WasmPublishPacketV3_1_1 or WasmPublishPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asPublish)]
    pub fn as_publish(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Publish(p) = &packet.inner {
                    let wrapper = WasmPublishPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Publish(p) = &packet.inner {
                    let wrapper = WasmPublishPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to CONNACK wrapper (version-aware)
    /// Returns WasmConnackPacketV3_1_1 or WasmConnackPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asConnack)]
    pub fn as_connack(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Connack(p) = &packet.inner {
                    let wrapper = WasmConnackPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Connack(p) = &packet.inner {
                    let wrapper = WasmConnackPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to SUBACK wrapper (version-aware)
    /// Returns WasmSubackPacketV3_1_1 or WasmSubackPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asSuback)]
    pub fn as_suback(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Suback(p) = &packet.inner {
                    let wrapper = WasmSubackPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Suback(p) = &packet.inner {
                    let wrapper = WasmSubackPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to UNSUBACK wrapper (version-aware)
    /// Returns WasmUnsubackPacketV3_1_1 or WasmUnsubackPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asUnsuback)]
    pub fn as_unsuback(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Unsuback(p) = &packet.inner {
                    let wrapper = WasmUnsubackPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Unsuback(p) = &packet.inner {
                    let wrapper = WasmUnsubackPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to PUBACK wrapper (version-aware)
    /// Returns WasmPubackPacketV3_1_1 or WasmPubackPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asPuback)]
    pub fn as_puback(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Puback(p) = &packet.inner {
                    let wrapper = WasmPubackPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Puback(p) = &packet.inner {
                    let wrapper = WasmPubackPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to PUBREC wrapper (version-aware)
    /// Returns WasmPubrecPacketV3_1_1 or WasmPubrecPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asPubrec)]
    pub fn as_pubrec(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Pubrec(p) = &packet.inner {
                    let wrapper = WasmPubrecPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Pubrec(p) = &packet.inner {
                    let wrapper = WasmPubrecPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to PUBREL wrapper (version-aware)
    /// Returns WasmPubrelPacketV3_1_1 or WasmPubrelPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asPubrel)]
    pub fn as_pubrel(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Pubrel(p) = &packet.inner {
                    let wrapper = WasmPubrelPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Pubrel(p) = &packet.inner {
                    let wrapper = WasmPubrelPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to PUBCOMP wrapper (version-aware)
    /// Returns WasmPubcompPacketV3_1_1 or WasmPubcompPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asPubcomp)]
    pub fn as_pubcomp(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Pubcomp(p) = &packet.inner {
                    let wrapper = WasmPubcompPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Pubcomp(p) = &packet.inner {
                    let wrapper = WasmPubcompPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to DISCONNECT wrapper (version-aware)
    /// Returns WasmDisconnectPacketV3_1_1 or WasmDisconnectPacketV5_0 based on client version
    #[wasm_bindgen(js_name = asDisconnect)]
    pub fn as_disconnect(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V3_1_1 => {
                if let mqtt::packet::Packet::V3_1_1Disconnect(p) = &packet.inner {
                    let wrapper = WasmDisconnectPacketV3_1_1 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Disconnect(p) = &packet.inner {
                    let wrapper = WasmDisconnectPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }

    /// Convert packet to AUTH wrapper (V5.0 only)
    /// Returns WasmAuthPacketV5_0 for V5.0 clients, null otherwise
    #[wasm_bindgen(js_name = asAuth)]
    pub fn as_auth(&self, packet: &WasmMqttPacket) -> JsValue {
        match self.version {
            mqtt::Version::V5_0 => {
                if let mqtt::packet::Packet::V5_0Auth(p) = &packet.inner {
                    let wrapper = WasmAuthPacketV5_0 { inner: p.clone() };
                    JsValue::from(wrapper)
                } else {
                    JsValue::NULL
                }
            }
            _ => JsValue::NULL,
        }
    }
}
