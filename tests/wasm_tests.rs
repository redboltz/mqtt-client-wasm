//! WASM interface tests
//!
//! Run with: wasm-pack test --node

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use mqtt_client_wasm::wasm::{WasmMqttConfig, WasmMqttPacket, WasmPacketType};

// ============================================================================
// WasmMqttConfig Tests
// ============================================================================

mod config_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_config_creation_v311() {
        // Default version is V3.1.1
        let options = js_sys::Object::new();
        let result = WasmMqttConfig::new(options.into());
        assert!(result.is_ok(), "Config creation failed: {:?}", result.err());
    }

    #[wasm_bindgen_test]
    fn test_config_creation_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"version".into(), &"5.0".into()).unwrap();
        js_sys::Reflect::set(&options, &"pingreqSendIntervalMs".into(), &30000u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"autoPubResponse".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"autoPingResponse".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"pingrespRecvTimeoutMs".into(), &5000u32.into()).unwrap();
        js_sys::Reflect::set(
            &options,
            &"connectionEstablishTimeoutMs".into(),
            &10000u32.into(),
        )
        .unwrap();
        js_sys::Reflect::set(&options, &"shutdownTimeoutMs".into(), &5000u32.into()).unwrap();

        let result = WasmMqttConfig::new(options.into());
        assert!(result.is_ok(), "Config creation failed: {:?}", result.err());
    }

    #[wasm_bindgen_test]
    fn test_config_version_aliases() {
        // Test "V5_0"
        let options1 = js_sys::Object::new();
        js_sys::Reflect::set(&options1, &"version".into(), &"V5_0".into()).unwrap();
        assert!(WasmMqttConfig::new(options1.into()).is_ok());

        // Test "v5.0"
        let options2 = js_sys::Object::new();
        js_sys::Reflect::set(&options2, &"version".into(), &"v5.0".into()).unwrap();
        assert!(WasmMqttConfig::new(options2.into()).is_ok());

        // Test "5"
        let options3 = js_sys::Object::new();
        js_sys::Reflect::set(&options3, &"version".into(), &"5".into()).unwrap();
        assert!(WasmMqttConfig::new(options3.into()).is_ok());
    }
}

// ============================================================================
// WasmMqttPacket V3.1.1 Factory Tests
// ============================================================================

mod v3_1_1_packet_factory_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_new_connect_v311() {
        let options = js_sys::Object::new();
        // Use camelCase field names as expected by serde
        js_sys::Reflect::set(&options, &"clientId".into(), &"test-client".into()).unwrap();
        js_sys::Reflect::set(&options, &"cleanSession".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"keepAlive".into(), &60u32.into()).unwrap();

        let result = WasmMqttPacket::new_connect_v311(options.into());
        assert!(
            result.is_ok(),
            "CONNECT creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Connect);
    }

    #[wasm_bindgen_test]
    fn test_new_publish_v311() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello world".into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"retain".into(), &false.into()).unwrap();

        let result = WasmMqttPacket::new_publish_v311(options.into());
        assert!(
            result.is_ok(),
            "PUBLISH creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Publish);
    }

    #[wasm_bindgen_test]
    fn test_new_subscribe_v311() {
        let subscription = js_sys::Object::new();
        js_sys::Reflect::set(&subscription, &"topic".into(), &"test/#".into()).unwrap();
        js_sys::Reflect::set(&subscription, &"qos".into(), &1u32.into()).unwrap();

        let subscriptions = js_sys::Array::new();
        subscriptions.push(&subscription);

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"subscriptions".into(), &subscriptions).unwrap();

        let result = WasmMqttPacket::new_subscribe_v311(options.into());
        assert!(
            result.is_ok(),
            "SUBSCRIBE creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Subscribe);
    }

    #[wasm_bindgen_test]
    fn test_new_unsubscribe_v311() {
        let topics = js_sys::Array::new();
        topics.push(&"test/topic".into());

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &2u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topics".into(), &topics).unwrap();

        let result = WasmMqttPacket::new_unsubscribe_v311(options.into());
        assert!(
            result.is_ok(),
            "UNSUBSCRIBE creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Unsubscribe);
    }

    #[wasm_bindgen_test]
    fn test_new_puback_v311() {
        let result = WasmMqttPacket::new_puback_v311(1);
        assert!(result.is_ok());

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Puback);
    }

    #[wasm_bindgen_test]
    fn test_new_pubrec_v311() {
        let result = WasmMqttPacket::new_pubrec_v311(2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrec);
    }

    #[wasm_bindgen_test]
    fn test_new_pubrel_v311() {
        let result = WasmMqttPacket::new_pubrel_v311(3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrel);
    }

    #[wasm_bindgen_test]
    fn test_new_pubcomp_v311() {
        let result = WasmMqttPacket::new_pubcomp_v311(4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubcomp);
    }

    #[wasm_bindgen_test]
    fn test_new_pingreq_v311() {
        let packet = WasmMqttPacket::new_pingreq_v311();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
    }

    #[wasm_bindgen_test]
    fn test_new_disconnect_v311() {
        let packet = WasmMqttPacket::new_disconnect_v311();
        assert_eq!(packet.packet_type(), WasmPacketType::Disconnect);
    }
}

// ============================================================================
// WasmMqttPacket V5.0 Factory Tests
// ============================================================================

mod v5_0_packet_factory_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_new_connect_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"clientId".into(), &"test-client-v5".into()).unwrap();
        js_sys::Reflect::set(&options, &"cleanStart".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"keepAlive".into(), &60u32.into()).unwrap();

        let result = WasmMqttPacket::new_connect_v50(options.into());
        assert!(
            result.is_ok(),
            "CONNECT V5 creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Connect);
    }

    #[wasm_bindgen_test]
    fn test_new_publish_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/v5/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello v5".into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &0u32.into()).unwrap();

        let result = WasmMqttPacket::new_publish_v50(options.into());
        assert!(
            result.is_ok(),
            "PUBLISH V5 creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Publish);
    }

    #[wasm_bindgen_test]
    fn test_new_publish_v50_with_properties() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/v5/props".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &r#"{"key":"value"}"#.into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"contentType".into(), &"application/json".into()).unwrap();
        js_sys::Reflect::set(&options, &"messageExpiryInterval".into(), &3600u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topicAlias".into(), &1u32.into()).unwrap();

        let result = WasmMqttPacket::new_publish_v50(options.into());
        assert!(
            result.is_ok(),
            "PUBLISH V5 with properties failed: {:?}",
            result.err()
        );
    }

    #[wasm_bindgen_test]
    fn test_new_subscribe_v50() {
        let subscription = js_sys::Object::new();
        js_sys::Reflect::set(&subscription, &"topic".into(), &"test/v5/#".into()).unwrap();
        js_sys::Reflect::set(&subscription, &"qos".into(), &2u32.into()).unwrap();

        let subscriptions = js_sys::Array::new();
        subscriptions.push(&subscription);

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"subscriptions".into(), &subscriptions).unwrap();

        let result = WasmMqttPacket::new_subscribe_v50(options.into());
        assert!(
            result.is_ok(),
            "SUBSCRIBE V5 creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Subscribe);
    }

    #[wasm_bindgen_test]
    fn test_new_puback_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = WasmMqttPacket::new_puback_v50(options.into());
        assert!(
            result.is_ok(),
            "PUBACK V5 creation failed: {:?}",
            result.err()
        );

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Puback);
    }

    #[wasm_bindgen_test]
    fn test_new_pingreq_v50() {
        let packet = WasmMqttPacket::new_pingreq_v50();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
    }

    #[wasm_bindgen_test]
    fn test_new_disconnect_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = WasmMqttPacket::new_disconnect_v50(options.into());
        assert!(result.is_ok());

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Disconnect);
    }

    #[wasm_bindgen_test]
    fn test_new_auth_v50() {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = WasmMqttPacket::new_auth_v50(options.into());
        assert!(result.is_ok());

        let packet = result.unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Auth);
    }
}

// ============================================================================
// Packet Type Enumeration Tests
// ============================================================================

mod packet_type_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_all_v311_packet_types() {
        // CONNECT
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"clientId".into(), &"test".into()).unwrap();
        let packet = WasmMqttPacket::new_connect_v311(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Connect);

        // PUBLISH
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"t".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"p".into()).unwrap();
        let packet = WasmMqttPacket::new_publish_v311(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Publish);

        // PUBACK
        assert_eq!(
            WasmMqttPacket::new_puback_v311(1).unwrap().packet_type(),
            WasmPacketType::Puback
        );

        // PUBREC
        assert_eq!(
            WasmMqttPacket::new_pubrec_v311(1).unwrap().packet_type(),
            WasmPacketType::Pubrec
        );

        // PUBREL
        assert_eq!(
            WasmMqttPacket::new_pubrel_v311(1).unwrap().packet_type(),
            WasmPacketType::Pubrel
        );

        // PUBCOMP
        assert_eq!(
            WasmMqttPacket::new_pubcomp_v311(1).unwrap().packet_type(),
            WasmPacketType::Pubcomp
        );

        // SUBSCRIBE
        let sub = js_sys::Object::new();
        js_sys::Reflect::set(&sub, &"topic".into(), &"#".into()).unwrap();
        js_sys::Reflect::set(&sub, &"qos".into(), &0u32.into()).unwrap();
        let subs = js_sys::Array::new();
        subs.push(&sub);
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"subscriptions".into(), &subs).unwrap();
        let packet = WasmMqttPacket::new_subscribe_v311(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Subscribe);

        // UNSUBSCRIBE
        let topics = js_sys::Array::new();
        topics.push(&"t".into());
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topics".into(), &topics).unwrap();
        let packet = WasmMqttPacket::new_unsubscribe_v311(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Unsubscribe);

        // PINGREQ
        assert_eq!(
            WasmMqttPacket::new_pingreq_v311().packet_type(),
            WasmPacketType::Pingreq
        );

        // DISCONNECT
        assert_eq!(
            WasmMqttPacket::new_disconnect_v311().packet_type(),
            WasmPacketType::Disconnect
        );
    }

    #[wasm_bindgen_test]
    fn test_v50_auth_packet() {
        // AUTH (V5.0 only)
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        let packet = WasmMqttPacket::new_auth_v50(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Auth);
    }
}

// ============================================================================
// Version-Aware Packet Creation Tests (WasmMqttClient methods)
// ============================================================================

mod version_aware_packet_tests {
    use super::*;
    use mqtt_client_wasm::wasm::WasmMqttClient;

    fn create_client_v311() -> WasmMqttClient {
        let options = js_sys::Object::new();
        // Default version is V3.1.1
        let config = WasmMqttConfig::new(options.into()).unwrap();
        WasmMqttClient::new(config)
    }

    fn create_client_v50() -> WasmMqttClient {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"version".into(), &"5.0".into()).unwrap();
        let config = WasmMqttConfig::new(options.into()).unwrap();
        WasmMqttClient::new(config)
    }

    // ------------------------------------------------------------------------
    // newConnectPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_connect_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"clientId".into(), &"test-v311".into()).unwrap();
        js_sys::Reflect::set(&options, &"cleanSession".into(), &true.into()).unwrap();

        let result = client.new_connect_packet(options.into());
        assert!(
            result.is_ok(),
            "newConnectPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Connect);
    }

    #[wasm_bindgen_test]
    fn test_new_connect_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"clientId".into(), &"test-v50".into()).unwrap();
        js_sys::Reflect::set(&options, &"cleanSession".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"sessionExpiryInterval".into(), &3600u32.into()).unwrap();

        let result = client.new_connect_packet(options.into());
        assert!(
            result.is_ok(),
            "newConnectPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Connect);
    }

    // ------------------------------------------------------------------------
    // newPublishPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_publish_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello".into()).unwrap();

        let result = client.new_publish_packet(options.into());
        assert!(
            result.is_ok(),
            "newPublishPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Publish);
    }

    #[wasm_bindgen_test]
    fn test_new_publish_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello v5".into()).unwrap();
        js_sys::Reflect::set(&options, &"contentType".into(), &"text/plain".into()).unwrap();

        let result = client.new_publish_packet(options.into());
        assert!(
            result.is_ok(),
            "newPublishPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Publish);
    }

    // ------------------------------------------------------------------------
    // newSubscribePacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_subscribe_packet_v311() {
        let client = create_client_v311();

        let subscription = js_sys::Object::new();
        js_sys::Reflect::set(&subscription, &"topic".into(), &"test/#".into()).unwrap();
        js_sys::Reflect::set(&subscription, &"qos".into(), &1u32.into()).unwrap();

        let subscriptions = js_sys::Array::new();
        subscriptions.push(&subscription);

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"subscriptions".into(), &subscriptions).unwrap();

        let result = client.new_subscribe_packet(options.into());
        assert!(
            result.is_ok(),
            "newSubscribePacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Subscribe);
    }

    #[wasm_bindgen_test]
    fn test_new_subscribe_packet_v50() {
        let client = create_client_v50();

        let subscription = js_sys::Object::new();
        js_sys::Reflect::set(&subscription, &"topic".into(), &"test/#".into()).unwrap();
        js_sys::Reflect::set(&subscription, &"qos".into(), &2u32.into()).unwrap();
        js_sys::Reflect::set(&subscription, &"noLocal".into(), &true.into()).unwrap();

        let subscriptions = js_sys::Array::new();
        subscriptions.push(&subscription);

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"subscriptions".into(), &subscriptions).unwrap();
        js_sys::Reflect::set(&options, &"subscriptionIdentifier".into(), &100u32.into()).unwrap();

        let result = client.new_subscribe_packet(options.into());
        assert!(
            result.is_ok(),
            "newSubscribePacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Subscribe);
    }

    // ------------------------------------------------------------------------
    // newUnsubscribePacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_unsubscribe_packet_v311() {
        let client = create_client_v311();

        let topics = js_sys::Array::new();
        topics.push(&"test/topic".into());

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topics".into(), &topics).unwrap();

        let result = client.new_unsubscribe_packet(options.into());
        assert!(
            result.is_ok(),
            "newUnsubscribePacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Unsubscribe);
    }

    #[wasm_bindgen_test]
    fn test_new_unsubscribe_packet_v50() {
        let client = create_client_v50();

        let topics = js_sys::Array::new();
        topics.push(&"test/topic".into());

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topics".into(), &topics).unwrap();

        let result = client.new_unsubscribe_packet(options.into());
        assert!(
            result.is_ok(),
            "newUnsubscribePacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Unsubscribe);
    }

    // ------------------------------------------------------------------------
    // newPubackPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_puback_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();

        let result = client.new_puback_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubackPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Puback);
    }

    #[wasm_bindgen_test]
    fn test_new_puback_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_puback_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubackPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Puback);
    }

    // ------------------------------------------------------------------------
    // newPubrecPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_pubrec_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &2u32.into()).unwrap();

        let result = client.new_pubrec_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubrecPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrec);
    }

    #[wasm_bindgen_test]
    fn test_new_pubrec_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &2u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_pubrec_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubrecPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrec);
    }

    // ------------------------------------------------------------------------
    // newPubrelPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_pubrel_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &3u32.into()).unwrap();

        let result = client.new_pubrel_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubrelPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrel);
    }

    #[wasm_bindgen_test]
    fn test_new_pubrel_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &3u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_pubrel_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubrelPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubrel);
    }

    // ------------------------------------------------------------------------
    // newPubcompPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_pubcomp_packet_v311() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &4u32.into()).unwrap();

        let result = client.new_pubcomp_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubcompPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubcomp);
    }

    #[wasm_bindgen_test]
    fn test_new_pubcomp_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &4u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_pubcomp_packet(options.into());
        assert!(
            result.is_ok(),
            "newPubcompPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Pubcomp);
    }

    // ------------------------------------------------------------------------
    // newPingreqPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_pingreq_packet_v311() {
        let client = create_client_v311();
        let packet = client.new_pingreq_packet();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
    }

    #[wasm_bindgen_test]
    fn test_new_pingreq_packet_v50() {
        let client = create_client_v50();
        let packet = client.new_pingreq_packet();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
    }

    // ------------------------------------------------------------------------
    // newDisconnectPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_disconnect_packet_v311() {
        let client = create_client_v311();

        // V3.1.1 ignores options
        let result = client.new_disconnect_packet(wasm_bindgen::JsValue::UNDEFINED);
        assert!(
            result.is_ok(),
            "newDisconnectPacket V3.1.1 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Disconnect);
    }

    #[wasm_bindgen_test]
    fn test_new_disconnect_packet_v311_with_options() {
        let client = create_client_v311();

        // V3.1.1 should ignore options
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_disconnect_packet(options.into());
        assert!(
            result.is_ok(),
            "newDisconnectPacket V3.1.1 with options failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Disconnect);
    }

    #[wasm_bindgen_test]
    fn test_new_disconnect_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"sessionExpiryInterval".into(), &0u32.into()).unwrap();

        let result = client.new_disconnect_packet(options.into());
        assert!(
            result.is_ok(),
            "newDisconnectPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Disconnect);
    }

    #[wasm_bindgen_test]
    fn test_new_disconnect_packet_v50_no_options() {
        let client = create_client_v50();

        // V5.0 with no options should use defaults
        let result = client.new_disconnect_packet(wasm_bindgen::JsValue::UNDEFINED);
        assert!(
            result.is_ok(),
            "newDisconnectPacket V5.0 no options failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Disconnect);
    }

    // ------------------------------------------------------------------------
    // newAuthPacket tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_new_auth_packet_v50() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(
            &options,
            &"authenticationMethod".into(),
            &"SCRAM-SHA-256".into(),
        )
        .unwrap();

        let result = client.new_auth_packet(options.into());
        assert!(
            result.is_ok(),
            "newAuthPacket V5.0 failed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().packet_type(), WasmPacketType::Auth);
    }

    #[wasm_bindgen_test]
    fn test_new_auth_packet_v311_should_fail() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let result = client.new_auth_packet(options.into());
        assert!(
            result.is_err(),
            "newAuthPacket should fail for V3.1.1 client"
        );
    }
}

// ============================================================================
// Packet Accessor Method Tests (via asXxx conversion using js_sys::Reflect)
// ============================================================================

mod packet_accessor_tests {
    use super::*;
    use mqtt_client_wasm::wasm::WasmMqttClient;
    use wasm_bindgen::JsValue;

    fn create_client_v311() -> WasmMqttClient {
        let options = js_sys::Object::new();
        let config = WasmMqttConfig::new(options.into()).unwrap();
        WasmMqttClient::new(config)
    }

    fn create_client_v50() -> WasmMqttClient {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"version".into(), &"5.0".into()).unwrap();
        let config = WasmMqttConfig::new(options.into()).unwrap();
        WasmMqttClient::new(config)
    }

    // Helper to get JS property as String
    fn get_string(obj: &JsValue, key: &str) -> Option<String> {
        js_sys::Reflect::get(obj, &key.into())
            .ok()
            .and_then(|v| v.as_string())
    }

    // Helper to get JS property as f64 (for numbers)
    fn get_number(obj: &JsValue, key: &str) -> Option<f64> {
        js_sys::Reflect::get(obj, &key.into())
            .ok()
            .and_then(|v| v.as_f64())
    }

    // Helper to get JS property as bool
    fn get_bool(obj: &JsValue, key: &str) -> Option<bool> {
        js_sys::Reflect::get(obj, &key.into())
            .ok()
            .and_then(|v| v.as_bool())
    }

    // ------------------------------------------------------------------------
    // V3.1.1 PUBLISH accessor tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_publish_v311_accessors() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello world".into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &1u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"retain".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"dup".into(), &false.into()).unwrap();
        js_sys::Reflect::set(&options, &"packetId".into(), &42u32.into()).unwrap();

        let packet = client.new_publish_packet(options.into()).unwrap();
        let publish = client.as_publish(&packet);
        assert!(!publish.is_null(), "as_publish should return non-null");

        // Access properties via JS reflection (simulating JavaScript access)
        assert_eq!(
            get_string(&publish, "topicName"),
            Some("test/topic".to_string())
        );
        assert_eq!(
            get_string(&publish, "payload"),
            Some("hello world".to_string())
        );
        assert_eq!(get_number(&publish, "qos"), Some(1.0));
        assert_eq!(get_bool(&publish, "retain"), Some(true));
        assert_eq!(get_bool(&publish, "dup"), Some(false));
        assert_eq!(get_number(&publish, "packetId"), Some(42.0));
    }

    #[wasm_bindgen_test]
    fn test_publish_v311_qos0_no_packet_id() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/qos0".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"qos0 message".into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &0u32.into()).unwrap();

        let packet = client.new_publish_packet(options.into()).unwrap();
        let publish = client.as_publish(&packet);

        assert_eq!(get_number(&publish, "qos"), Some(0.0));
        // packetId should be undefined for QoS 0
        let packet_id = js_sys::Reflect::get(&publish, &"packetId".into()).unwrap();
        assert!(
            packet_id.is_undefined(),
            "packetId should be undefined for QoS 0"
        );
    }

    #[wasm_bindgen_test]
    fn test_publish_v311_binary_payload() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/binary".into()).unwrap();

        let binary_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE];
        let uint8_array = js_sys::Uint8Array::from(&binary_data[..]);
        js_sys::Reflect::set(&options, &"payloadBytes".into(), &uint8_array).unwrap();

        let packet = client.new_publish_packet(options.into()).unwrap();
        let publish = client.as_publish(&packet);

        // payload should be undefined for non-UTF8 data
        let payload = js_sys::Reflect::get(&publish, &"payload".into()).unwrap();
        assert!(
            payload.is_undefined(),
            "payload should be undefined for non-UTF8 data"
        );

        // topicName should still work
        assert_eq!(
            get_string(&publish, "topicName"),
            Some("test/binary".to_string())
        );
    }

    // ------------------------------------------------------------------------
    // V3.1.1 PUBACK/PUBREC/PUBREL/PUBCOMP accessor tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_puback_v311_accessor() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &123u32.into()).unwrap();

        let packet = client.new_puback_packet(options.into()).unwrap();
        let puback = client.as_puback(&packet);

        assert_eq!(get_number(&puback, "packetId"), Some(123.0));
    }

    #[wasm_bindgen_test]
    fn test_pubrec_v311_accessor() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &456u32.into()).unwrap();

        let packet = client.new_pubrec_packet(options.into()).unwrap();
        let pubrec = client.as_pubrec(&packet);

        assert_eq!(get_number(&pubrec, "packetId"), Some(456.0));
    }

    #[wasm_bindgen_test]
    fn test_pubrel_v311_accessor() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &789u32.into()).unwrap();

        let packet = client.new_pubrel_packet(options.into()).unwrap();
        let pubrel = client.as_pubrel(&packet);

        assert_eq!(get_number(&pubrel, "packetId"), Some(789.0));
    }

    #[wasm_bindgen_test]
    fn test_pubcomp_v311_accessor() {
        let client = create_client_v311();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1000u32.into()).unwrap();

        let packet = client.new_pubcomp_packet(options.into()).unwrap();
        let pubcomp = client.as_pubcomp(&packet);

        assert_eq!(get_number(&pubcomp, "packetId"), Some(1000.0));
    }

    // ------------------------------------------------------------------------
    // V5.0 PUBLISH accessor tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_publish_v50_accessors() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/v5/topic".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"hello v5".into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &2u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"retain".into(), &true.into()).unwrap();
        js_sys::Reflect::set(&options, &"packetId".into(), &100u32.into()).unwrap();

        let packet = client.new_publish_packet(options.into()).unwrap();
        let publish = client.as_publish(&packet);

        assert_eq!(
            get_string(&publish, "topicName"),
            Some("test/v5/topic".to_string())
        );
        assert_eq!(
            get_string(&publish, "payload"),
            Some("hello v5".to_string())
        );
        assert_eq!(get_number(&publish, "qos"), Some(2.0));
        assert_eq!(get_bool(&publish, "retain"), Some(true));
        assert_eq!(get_number(&publish, "packetId"), Some(100.0));
    }

    #[wasm_bindgen_test]
    fn test_publish_v50_with_properties() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test/v5/props".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &r#"{"key":"value"}"#.into()).unwrap();
        js_sys::Reflect::set(&options, &"qos".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"contentType".into(), &"application/json".into()).unwrap();
        js_sys::Reflect::set(&options, &"messageExpiryInterval".into(), &3600u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"topicAlias".into(), &5u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"responseTopic".into(), &"response/topic".into()).unwrap();

        let packet = client.new_publish_packet(options.into()).unwrap();
        let publish = client.as_publish(&packet);

        assert_eq!(
            get_string(&publish, "contentType"),
            Some("application/json".to_string())
        );
        assert_eq!(get_number(&publish, "messageExpiryInterval"), Some(3600.0));
        assert_eq!(get_number(&publish, "topicAlias"), Some(5.0));
        assert_eq!(
            get_string(&publish, "responseTopic"),
            Some("response/topic".to_string())
        );
    }

    // ------------------------------------------------------------------------
    // V5.0 PUBACK/PUBREC/PUBREL/PUBCOMP accessor tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_puback_v50_accessor() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &200u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonString".into(), &"Success".into()).unwrap();

        let packet = client.new_puback_packet(options.into()).unwrap();
        let puback = client.as_puback(&packet);

        assert_eq!(get_number(&puback, "packetId"), Some(200.0));
        assert_eq!(get_number(&puback, "reasonCode"), Some(0.0));
        assert_eq!(
            get_string(&puback, "reasonString"),
            Some("Success".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn test_pubrec_v50_accessor() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &300u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &16u32.into()).unwrap();

        let packet = client.new_pubrec_packet(options.into()).unwrap();
        let pubrec = client.as_pubrec(&packet);

        assert_eq!(get_number(&pubrec, "packetId"), Some(300.0));
        assert_eq!(get_number(&pubrec, "reasonCode"), Some(16.0));
    }

    #[wasm_bindgen_test]
    fn test_pubrel_v50_accessor() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &400u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();

        let packet = client.new_pubrel_packet(options.into()).unwrap();
        let pubrel = client.as_pubrel(&packet);

        assert_eq!(get_number(&pubrel, "packetId"), Some(400.0));
        assert_eq!(get_number(&pubrel, "reasonCode"), Some(0.0));
    }

    #[wasm_bindgen_test]
    fn test_pubcomp_v50_accessor() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &500u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(&options, &"reasonString".into(), &"Complete".into()).unwrap();

        let packet = client.new_pubcomp_packet(options.into()).unwrap();
        let pubcomp = client.as_pubcomp(&packet);

        assert_eq!(get_number(&pubcomp, "packetId"), Some(500.0));
        assert_eq!(get_number(&pubcomp, "reasonCode"), Some(0.0));
        assert_eq!(
            get_string(&pubcomp, "reasonString"),
            Some("Complete".to_string())
        );
    }

    // ------------------------------------------------------------------------
    // V5.0 AUTH accessor tests
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_auth_v50_accessor() {
        let client = create_client_v50();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        js_sys::Reflect::set(
            &options,
            &"authenticationMethod".into(),
            &"SCRAM-SHA-256".into(),
        )
        .unwrap();
        js_sys::Reflect::set(&options, &"reasonString".into(), &"Auth success".into()).unwrap();

        let auth_data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let uint8_array = js_sys::Uint8Array::from(&auth_data[..]);
        js_sys::Reflect::set(&options, &"authenticationData".into(), &uint8_array).unwrap();

        let packet = client.new_auth_packet(options.into()).unwrap();
        let auth = client.as_auth(&packet);

        assert_eq!(get_number(&auth, "reasonCode"), Some(0.0));
        assert_eq!(
            get_string(&auth, "authenticationMethod"),
            Some("SCRAM-SHA-256".to_string())
        );
        assert_eq!(
            get_string(&auth, "reasonString"),
            Some("Auth success".to_string())
        );
    }

    // ------------------------------------------------------------------------
    // Type mismatch tests (as_xxx returns null for wrong packet type)
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_as_publish_returns_null_for_wrong_type() {
        let client = create_client_v311();

        // Create a PUBACK packet
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"packetId".into(), &1u32.into()).unwrap();
        let packet = client.new_puback_packet(options.into()).unwrap();

        // Try to convert to PUBLISH - should return null
        let result = client.as_publish(&packet);
        assert!(
            result.is_null(),
            "as_publish should return null for PUBACK packet"
        );
    }

    #[wasm_bindgen_test]
    fn test_as_auth_returns_null_for_v311() {
        let client = create_client_v311();

        // Create a PUBLISH packet (V3.1.1)
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"topicName".into(), &"test".into()).unwrap();
        js_sys::Reflect::set(&options, &"payload".into(), &"data".into()).unwrap();
        let packet = client.new_publish_packet(options.into()).unwrap();

        // as_auth should return null for V3.1.1 client
        let result = client.as_auth(&packet);
        assert!(
            result.is_null(),
            "as_auth should return null for V3.1.1 client"
        );
    }

    // ------------------------------------------------------------------------
    // PINGREQ/DISCONNECT tests (no additional accessors)
    // ------------------------------------------------------------------------

    #[wasm_bindgen_test]
    fn test_pingreq_v311_packet_type() {
        let client = create_client_v311();
        let packet = client.new_pingreq_packet();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
        // packet_type_string returns debug format, just verify it contains "pingreq"
        assert!(packet
            .packet_type_string()
            .to_lowercase()
            .contains("pingreq"));
    }

    #[wasm_bindgen_test]
    fn test_pingreq_v50_packet_type() {
        let client = create_client_v50();
        let packet = client.new_pingreq_packet();
        assert_eq!(packet.packet_type(), WasmPacketType::Pingreq);
        assert!(packet
            .packet_type_string()
            .to_lowercase()
            .contains("pingreq"));
    }

    #[wasm_bindgen_test]
    fn test_disconnect_v311_packet_type() {
        let client = create_client_v311();
        let packet = client.new_disconnect_packet(JsValue::UNDEFINED).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Disconnect);
        assert!(packet
            .packet_type_string()
            .to_lowercase()
            .contains("disconnect"));
    }

    #[wasm_bindgen_test]
    fn test_disconnect_v50_packet_type() {
        let client = create_client_v50();
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"reasonCode".into(), &0u32.into()).unwrap();
        let packet = client.new_disconnect_packet(options.into()).unwrap();
        assert_eq!(packet.packet_type(), WasmPacketType::Disconnect);
    }
}
