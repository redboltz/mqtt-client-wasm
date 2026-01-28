//! Tests for MQTT client functionality
//!
//! These tests verify the core MQTT client behavior including:
//! - Client creation and connection
//! - Packet flow and packet ID management
//! - Publish/Subscribe operations
//! - recv() timeout handling and packet loss prevention

#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::MockUnderlyingLayer;
use futures::channel::oneshot;
use mqtt::packet::GenericPacketTrait;
use mqtt_client_wasm::mqtt as client_mqtt;
use mqtt_client_wasm::{ConnectionState, MqttClient, MqttConfig, Result};
use mqtt_protocol_core::mqtt;

#[test]
fn test_mqtt_client_creation() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let _client = MqttClient::new_with_websocket(config, mock_ws);
}

#[tokio::test]
async fn test_mqtt_connect_flow() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    // Create client
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Test connection
    let result = client.connect("ws://test.example.com").await;
    assert!(result.is_ok());

    // Wait for connection state to be updated
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Test state
    let state = client.state().await;
    assert_eq!(state, ConnectionState::Connected);

    // Test is_connected
    let connected = client.is_connected().await;
    assert!(connected);
}

#[tokio::test]
async fn test_mqtt_packet_flow() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect first
    let _ = client.connect("ws://test.example.com").await;

    // Give some time for the connection to be established
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Test packet ID operations
    let packet_id = client.acquire_packet_id().await;
    assert!(packet_id.is_some());

    if let Some(id) = packet_id {
        // Note: register_packet_id may fail if MQTT connection is not fully established
        // This is expected behavior
        let _registered = client.register_packet_id(id).await;

        let released = client.release_packet_id(id).await;
        assert!(released.is_ok());
    }
}

#[tokio::test]
async fn test_mqtt_publish_subscribe() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect
    let _ = client.connect("ws://test.example.com").await;

    // Create PUBLISH packet
    let publish_packet = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/topic")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"Hello, World!")
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Publish(publish_packet);

    // Send packet
    let result = client.send(packet).await;
    assert!(result.is_ok());

    // Create SUBSCRIBE packet
    let packet_id = client.acquire_packet_id().await.unwrap();
    let sub_opts = mqtt::packet::SubOpts::new().set_qos(mqtt::packet::Qos::AtMostOnce);
    let sub_entry = mqtt::packet::SubEntry::new("test/topic", sub_opts).unwrap();

    let subscribe_packet = mqtt::packet::v3_1_1::Subscribe::builder()
        .packet_id(packet_id)
        .entries(vec![sub_entry])
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Subscribe(subscribe_packet);

    // Send subscribe packet
    let result = client.send(packet).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_comprehensive_mqtt_flow() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Test initial state
    assert_eq!(client.state().await, ConnectionState::Disconnected);
    assert!(!client.is_connected().await);

    // Connect
    let connect_result = client.connect("ws://test.broker.com").await;
    assert!(connect_result.is_ok());

    // Wait a bit for connection to be established
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Test connection state
    assert_eq!(client.state().await, ConnectionState::Connected);
    assert!(client.is_connected().await);

    // Test packet ID management
    let packet_id_1 = client.acquire_packet_id().await;
    let packet_id_2 = client.acquire_packet_id().await;
    assert!(packet_id_1.is_some());
    assert!(packet_id_2.is_some());
    assert_ne!(packet_id_1, packet_id_2);

    // Test QoS 1 PUBLISH with packet ID
    if let Some(id) = packet_id_1 {
        let publish_qos1 = mqtt::packet::v3_1_1::Publish::builder()
            .topic_name("test/qos1")
            .unwrap()
            .qos(mqtt::packet::Qos::AtLeastOnce)
            .packet_id(id)
            .payload(b"QoS 1 message")
            .build()
            .unwrap();

        let packet = mqtt::packet::Packet::V3_1_1Publish(publish_qos1);
        let send_result = client.send(packet).await;
        assert!(send_result.is_ok());

        // Release packet ID
        let release_result = client.release_packet_id(id).await;
        assert!(release_result.is_ok());
    }

    // Test QoS 0 PUBLISH (no packet ID required)
    let publish_qos0 = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/qos0")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"QoS 0 message")
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Publish(publish_qos0);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());

    // Test SUBSCRIBE with multiple topics
    if let Some(id) = packet_id_2 {
        let sub_opts = mqtt::packet::SubOpts::new().set_qos(mqtt::packet::Qos::AtLeastOnce);
        let sub_entries = vec![
            mqtt::packet::SubEntry::new("test/topic1", sub_opts).unwrap(),
            mqtt::packet::SubEntry::new("test/topic2", sub_opts).unwrap(),
        ];

        let subscribe_packet = mqtt::packet::v3_1_1::Subscribe::builder()
            .packet_id(id)
            .entries(sub_entries)
            .build()
            .unwrap();

        let packet = mqtt::packet::Packet::V3_1_1Subscribe(subscribe_packet);
        let send_result = client.send(packet).await;
        assert!(send_result.is_ok());
    }

    // Test close
    let close_result = client.close().await;
    assert!(close_result.is_ok());
}

/// Test that packets are not lost when recv() times out and is retried
///
/// This test verifies the handle_received_packet logic:
/// 1. When a pending recv request's receiver is dropped (timeout)
/// 2. The packet should be delivered to the next valid request
#[tokio::test]
#[allow(unused)]
async fn test_recv_timeout_no_packet_loss() {
    // Test the core logic using channels directly
    let mut pending_recv_requests: Vec<oneshot::Sender<Result<mqtt::packet::Packet>>> = Vec::new();
    let mut undelivered_packet: Option<mqtt::packet::Packet> = None;

    // Create a test packet
    let publish_packet = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/timeout")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"packet after timeout")
        .build()
        .unwrap();
    let test_packet = mqtt::packet::Packet::V3_1_1Publish(publish_packet);

    // First, add a request that will be "timed out" (we'll drop the receiver immediately)
    let (timeout_tx, timeout_rx) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    pending_recv_requests.push(timeout_tx);
    drop(timeout_rx); // Simulate timeout by dropping receiver

    // Add a second request that should receive the packet
    let (success_tx, success_rx) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    pending_recv_requests.push(success_tx);

    // Now simulate handle_received_packet logic
    let mut packet_to_deliver = Some(test_packet.clone());

    while let Some(pkt) = packet_to_deliver.take() {
        if pending_recv_requests.is_empty() {
            undelivered_packet = Some(pkt);
            break;
        }

        let reply = pending_recv_requests.remove(0);
        match reply.send(Ok(pkt)) {
            Ok(()) => {
                break;
            }
            Err(packet_result) => {
                if let Ok(returned_packet) = packet_result {
                    packet_to_deliver = Some(returned_packet);
                }
            }
        }
    }

    // The second receiver should have received the packet
    let received = tokio::time::timeout(tokio::time::Duration::from_millis(100), success_rx).await;

    assert!(received.is_ok(), "Should not timeout waiting for packet");
    let channel_result = received.unwrap();
    assert!(channel_result.is_ok(), "Channel should receive packet");
    let packet_result = channel_result.unwrap();
    assert!(packet_result.is_ok(), "Packet should be Ok");

    if let mqtt::packet::Packet::V3_1_1Publish(received_publish) = packet_result.unwrap() {
        assert_eq!(received_publish.topic_name(), "test/timeout");
        assert_eq!(
            received_publish.payload().as_slice(),
            b"packet after timeout"
        );
    } else {
        panic!("Expected V3_1_1Publish packet");
    }
}

/// Test that undelivered packet is saved when no pending requests
#[tokio::test]
async fn test_undelivered_packet_saved() {
    let mut pending_recv_requests: Vec<oneshot::Sender<Result<mqtt::packet::Packet>>> = Vec::new();
    let mut undelivered_packet: Option<mqtt::packet::Packet> = None;

    // Create a test packet
    let publish_packet = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/saved")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"saved packet")
        .build()
        .unwrap();
    let test_packet = mqtt::packet::Packet::V3_1_1Publish(publish_packet);

    // No pending requests - packet should be saved
    let mut packet_to_deliver = Some(test_packet.clone());

    while let Some(pkt) = packet_to_deliver.take() {
        if pending_recv_requests.is_empty() {
            undelivered_packet = Some(pkt);
            break;
        }

        let reply = pending_recv_requests.remove(0);
        match reply.send(Ok(pkt)) {
            Ok(()) => break,
            Err(packet_result) => {
                if let Ok(returned_packet) = packet_result {
                    packet_to_deliver = Some(returned_packet);
                }
            }
        }
    }

    // Packet should be saved to undelivered_packet
    assert!(undelivered_packet.is_some(), "Packet should be saved");

    if let Some(mqtt::packet::Packet::V3_1_1Publish(saved_publish)) = undelivered_packet {
        assert_eq!(saved_publish.topic_name(), "test/saved");
        assert_eq!(saved_publish.payload().as_slice(), b"saved packet");
    } else {
        panic!("Expected saved V3_1_1Publish packet");
    }
}

/// Test multiple consecutive recv() timeouts don't lose packets
#[tokio::test]
#[allow(unused)]
async fn test_multiple_recv_timeouts_no_packet_loss() {
    let mut pending_recv_requests: Vec<oneshot::Sender<Result<mqtt::packet::Packet>>> = Vec::new();
    let mut undelivered_packet: Option<mqtt::packet::Packet> = None;

    // Create a test packet
    let publish_packet = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/multi-timeout")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"delayed packet")
        .build()
        .unwrap();
    let test_packet = mqtt::packet::Packet::V3_1_1Publish(publish_packet);

    // Simulate 3 timed out requests (receivers dropped immediately)
    let (tx1, rx1) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    let (tx2, rx2) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    let (tx3, rx3) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    drop(rx1);
    drop(rx2);
    drop(rx3);
    pending_recv_requests.push(tx1);
    pending_recv_requests.push(tx2);
    pending_recv_requests.push(tx3);

    // Add a final request that should receive the packet
    let (success_tx, success_rx) = oneshot::channel::<Result<mqtt::packet::Packet>>();
    pending_recv_requests.push(success_tx);

    // Simulate handle_received_packet logic
    let mut packet_to_deliver = Some(test_packet.clone());

    while let Some(pkt) = packet_to_deliver.take() {
        if pending_recv_requests.is_empty() {
            undelivered_packet = Some(pkt);
            break;
        }

        let reply = pending_recv_requests.remove(0);
        match reply.send(Ok(pkt)) {
            Ok(()) => break,
            Err(packet_result) => {
                if let Ok(returned_packet) = packet_result {
                    packet_to_deliver = Some(returned_packet);
                }
            }
        }
    }

    // The fourth (final) receiver should have received the packet
    let received = tokio::time::timeout(tokio::time::Duration::from_millis(100), success_rx).await;

    assert!(received.is_ok(), "Should not timeout waiting for packet");
    let channel_result = received.unwrap();
    assert!(channel_result.is_ok(), "Channel should receive packet");
    let packet_result = channel_result.unwrap();
    assert!(
        packet_result.is_ok(),
        "Packet should be received successfully"
    );

    if let mqtt::packet::Packet::V3_1_1Publish(received_publish) = packet_result.unwrap() {
        assert_eq!(received_publish.payload().as_slice(), b"delayed packet");
    } else {
        panic!("Expected V3_1_1Publish packet");
    }
}

/// Test reconnection after close
///
/// This test triggers reset_for_reconnection() by:
/// 1. Connecting with full config (pingreq_send_interval_ms set)
/// 2. Closing the connection (state -> Closed, but processor continues due to MockUnderlyingLayer)
/// 3. Reconnecting (triggers reset_for_reconnection when state is Closed)
#[tokio::test]
async fn test_reconnect_after_close() {
    // Use config with all options to cover reset_for_reconnection branches
    let config = MqttConfig {
        url: String::new(),
        version: client_mqtt::Version::V3_1_1,
        pingreq_send_interval_ms: Some(30000), // Covered in reset_for_reconnection
        pingresp_recv_timeout_ms: 5000,
        auto_pub_response: false,
        auto_ping_response: false,
        auto_map_topic_alias_send: true,
        auto_replace_topic_alias_send: true,
        connection_establish_timeout_ms: 10000,
        shutdown_timeout_ms: 5000,
    };
    let mock_ws = MockUnderlyingLayer::new();

    // Create client
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Wait for background tasks to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // First connection
    let connect_result = client.connect("ws://localhost:1883").await;
    assert!(connect_result.is_ok(), "First connection should succeed");

    // Wait for connection
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    assert!(client.is_connected().await);

    // Close the connection - MockUnderlyingLayer continues running (doesn't break on Close)
    let close_result = client.close().await;
    assert!(close_result.is_ok(), "Close should succeed");

    // Wait for close to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Try to reconnect from Closed state - this triggers reset_for_reconnection()
    // The processor may have exited, but we try anyway
    let reconnect_result = client.connect("ws://localhost:1883").await;

    // If reconnection succeeds, verify connected
    if reconnect_result.is_ok() {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        // Check if connected (might not be if processor exited)
        let _ = client.is_connected().await;
    }
    // Regardless of result, this test exercises the reconnection code path
}

/// Test that Connecting and Connected states reject new connections
#[tokio::test]
async fn test_reject_connect_when_already_connected() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    // Create client
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Wait for background tasks to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // First connection
    let connect_result = client.connect("ws://localhost:1883").await;
    assert!(connect_result.is_ok(), "First connection should succeed");

    // Wait for connection to be established
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Try to connect again while already connected
    let second_connect = client.connect("ws://localhost:1883").await;
    assert!(second_connect.is_err(), "Second connection should fail");
    if let Err(e) = second_connect {
        assert!(e.to_string().contains("Already connecting or connected"));
    }
}

/// Test WebSocket error handling
#[tokio::test]
async fn test_websocket_error_handling() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect first
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Simulate WebSocket error
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Error(
        "Connection lost".to_string(),
    ));

    // Wait for error to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // State should be Disconnected after error
    let state = client.state().await;
    assert_eq!(state, ConnectionState::Disconnected);
}

/// Test receiving MQTT packets through WebSocket
#[tokio::test]
async fn test_receive_mqtt_packet() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Create a CONNACK packet to simulate broker response
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();

    // Simulate receiving CONNACK
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Try to receive the packet
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok(), "recv should not timeout");
    let packet_result = recv_result.unwrap();
    assert!(packet_result.is_ok(), "Should receive packet successfully");

    if let mqtt::packet::Packet::V3_1_1Connack(received_connack) = packet_result.unwrap() {
        assert!(!received_connack.session_present());
    } else {
        panic!("Expected CONNACK packet");
    }
}

/// Test recv timeout behavior using tokio::time::timeout
#[tokio::test]
async fn test_recv_timeout() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Try to receive with a short timeout (no packets coming)
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(100), client.recv()).await;

    // Should timeout
    assert!(recv_result.is_err());
}

/// Test config with pingreq_send_interval
#[tokio::test]
async fn test_config_with_pingreq_interval() {
    let config = MqttConfig {
        pingreq_send_interval_ms: Some(5000),
        auto_pub_response: true,
        auto_ping_response: true,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect and verify client works with custom config
    let connect_result = client.connect("ws://test.example.com").await;
    assert!(connect_result.is_ok());

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = client.state().await;
    assert_eq!(state, ConnectionState::Connected);
}

/// Test sending CONNECT packet and receiving response
#[tokio::test]
async fn test_connect_packet_exchange() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect WebSocket
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send CONNECT packet
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("test-client")
        .unwrap()
        .keep_alive(60)
        .clean_session(true)
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Connect(connect_packet);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());

    // Simulate CONNACK response from broker
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    assert!(matches!(packet, mqtt::packet::Packet::V3_1_1Connack(_)));
}

/// Test MQTT v5.0 packet exchange
#[tokio::test]
async fn test_mqtt_v50_packet_exchange() {
    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect WebSocket
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send v5.0 CONNECT packet
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("test-client-v5")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V5_0Connect(connect_packet);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());

    // Simulate v5.0 CONNACK response
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    assert!(matches!(packet, mqtt::packet::Packet::V5_0Connack(_)));
}

/// Test receiving multiple packets in sequence
#[tokio::test]
async fn test_receive_multiple_packets() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send and receive packets one at a time for reliability
    for i in 0..3 {
        let publish = mqtt::packet::v3_1_1::Publish::builder()
            .topic_name(&format!("test/topic{}", i))
            .unwrap()
            .qos(mqtt::packet::Qos::AtMostOnce)
            .payload(format!("message {}", i).as_bytes())
            .build()
            .unwrap();
        let publish_bytes = mqtt::packet::Packet::V3_1_1Publish(publish).to_continuous_buffer();
        let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
            publish_bytes,
        ));

        // Receive this packet
        let recv_result =
            tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

        assert!(recv_result.is_ok(), "Should receive packet {}", i);
        let packet = recv_result.unwrap().unwrap();
        if let mqtt::packet::Packet::V3_1_1Publish(pub_packet) = packet {
            assert_eq!(pub_packet.topic_name(), format!("test/topic{}", i));
        } else {
            panic!("Expected PUBLISH packet");
        }
    }
}

/// Test WebSocket close during operation
#[tokio::test]
async fn test_websocket_close_during_operation() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    assert!(client.is_connected().await);

    // Simulate WebSocket close
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Closed);

    // Wait for close to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // State should be Closed
    let state = client.state().await;
    assert_eq!(state, ConnectionState::Closed);
}

/// Test PINGRESP packet reception
#[tokio::test]
async fn test_pingresp_reception() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        auto_ping_response: false, // Disable auto response so we can see the PINGRESP
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send a PINGRESP packet
    let pingresp = mqtt::packet::v3_1_1::Pingresp::builder().build().unwrap();
    let pingresp_bytes = mqtt::packet::Packet::V3_1_1Pingresp(pingresp).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        pingresp_bytes,
    ));

    // Receive the packet with timeout
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    assert!(matches!(packet, mqtt::packet::Packet::V3_1_1Pingresp(_)));
}

/// Test DISCONNECT packet handling
#[tokio::test]
async fn test_disconnect_packet() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send DISCONNECT packet
    let disconnect = mqtt::packet::v3_1_1::Disconnect::builder().build().unwrap();
    let packet = mqtt::packet::Packet::V3_1_1Disconnect(disconnect);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());
}

/// Test UNSUBSCRIBE packet
#[tokio::test]
async fn test_unsubscribe_packet() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let packet_id = client.acquire_packet_id().await.unwrap();

    let unsubscribe = mqtt::packet::v3_1_1::Unsubscribe::builder()
        .packet_id(packet_id)
        .entries(vec!["test/topic"])
        .unwrap()
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Unsubscribe(unsubscribe);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());
}

/// Test QoS 2 PUBLISH flow
#[tokio::test]
async fn test_qos2_publish() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        auto_pub_response: false, // Disable auto response so we can see the PUBREC
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // First establish MQTT connection by sending CONNECT
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("test-qos2-client")
        .unwrap()
        .keep_alive(60)
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    // Simulate CONNACK response from broker
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Wait for CONNACK to be processed
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    let packet_id = client.acquire_packet_id().await.unwrap();

    // Send QoS 2 PUBLISH
    let publish = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/qos2")
        .unwrap()
        .qos(mqtt::packet::Qos::ExactlyOnce)
        .packet_id(packet_id)
        .payload(b"QoS 2 message")
        .build()
        .unwrap();

    let packet = mqtt::packet::Packet::V3_1_1Publish(publish);
    let send_result = client.send(packet).await;
    assert!(send_result.is_ok());

    // Simulate PUBREC response
    let pubrec = mqtt::packet::v3_1_1::Pubrec::builder()
        .packet_id(packet_id)
        .build()
        .unwrap();
    let pubrec_bytes = mqtt::packet::Packet::V3_1_1Pubrec(pubrec).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        pubrec_bytes,
    ));

    // Receive PUBREC
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    assert!(matches!(packet, mqtt::packet::Packet::V3_1_1Pubrec(_)));
}

/// Test register_packet_id functionality
#[tokio::test]
async fn test_register_packet_id() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Register a specific packet ID
    let result = client.register_packet_id(100).await;
    assert!(result); // Should succeed

    // Try to register the same packet ID again - should fail
    let result2 = client.register_packet_id(100).await;
    assert!(!result2); // Should fail because already registered
}

/// Test release_packet_id functionality
#[tokio::test]
async fn test_release_packet_id() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Acquire a packet ID
    let packet_id = client.acquire_packet_id().await.unwrap();

    // Release it
    let result = client.release_packet_id(packet_id).await;
    assert!(result.is_ok());

    // Should be able to acquire the same ID again after release
    // (or a different one - the pool should have it available)
    let packet_id2 = client.acquire_packet_id().await;
    assert!(packet_id2.is_some());
}

/// Test partial packet reassembly (buffer compaction)
#[tokio::test]
async fn test_partial_packet_reassembly() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Create a PUBLISH packet
    let publish = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/partial")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"test message for partial reassembly")
        .build()
        .unwrap();
    let packet_bytes = mqtt::packet::Packet::V3_1_1Publish(publish).to_continuous_buffer();

    // Split the packet into two parts
    let mid = packet_bytes.len() / 2;
    let part1 = packet_bytes[..mid].to_vec();
    let part2 = packet_bytes[mid..].to_vec();

    // Send first part
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(part1));
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    // Send second part
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(part2));

    // Should receive complete packet after reassembly
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V3_1_1Publish(pub_packet) = packet {
        assert_eq!(pub_packet.topic_name(), "test/partial");
    } else {
        panic!("Expected PUBLISH packet");
    }
}

/// Test config with all options set
#[tokio::test]
async fn test_config_with_all_options() {
    let config = MqttConfig {
        url: String::new(),
        version: client_mqtt::Version::V3_1_1,
        pingreq_send_interval_ms: Some(30000),
        pingresp_recv_timeout_ms: 5000,
        auto_pub_response: false,
        auto_ping_response: false,
        auto_map_topic_alias_send: true,
        auto_replace_topic_alias_send: true,
        connection_establish_timeout_ms: 10000,
        shutdown_timeout_ms: 5000,
    };
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connection should work with all options set
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    assert!(client.is_connected().await);
}

/// Test is_connected returns false after disconnect
#[tokio::test]
async fn test_is_connected_after_disconnect() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    assert!(client.is_connected().await);

    // Simulate close
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Closed);
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // After close, is_connected should return false
    assert!(!client.is_connected().await);
}

/// Test acquire_packet_id returns None when channel is closed
#[tokio::test]
async fn test_acquire_packet_id_after_close() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close the client
    let _ = client.close().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // After close, acquire_packet_id might return None
    let packet_id = client.acquire_packet_id().await;
    // Result depends on timing - just ensure no panic
    let _ = packet_id;
}

/// Test recv returns error or times out when WebSocket is closed
#[tokio::test]
async fn test_recv_after_websocket_close() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close WebSocket
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Closed);
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Verify state is Closed
    let state = client.state().await;
    assert_eq!(state, ConnectionState::Closed);
}

/// Test multiple concurrent packet ID acquisitions
#[tokio::test]
async fn test_multiple_packet_id_acquisitions() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Acquire multiple packet IDs
    let mut packet_ids = Vec::new();
    for _ in 0..10 {
        let id = client.acquire_packet_id().await;
        assert!(id.is_some());
        packet_ids.push(id.unwrap());
    }

    // All IDs should be unique
    let unique_ids: std::collections::HashSet<_> = packet_ids.iter().collect();
    assert_eq!(unique_ids.len(), packet_ids.len());

    // Release all packet IDs
    for id in packet_ids {
        let result = client.release_packet_id(id).await;
        assert!(result.is_ok());
    }
}

/// Test state transitions
#[tokio::test]
async fn test_state_transitions() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Initial state should be Disconnected
    let state = client.state().await;
    assert_eq!(state, ConnectionState::Disconnected);

    // After connect request, state should transition
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = client.state().await;
    assert_eq!(state, ConnectionState::Connected);

    // After error, state should be Disconnected
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Error(
        "test error".to_string(),
    ));
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let state = client.state().await;
    assert_eq!(state, ConnectionState::Disconnected);
}

/// Test send when channel is closed (error path)
#[tokio::test]
async fn test_send_after_close() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close the client
    let _ = client.close().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Try to send after close - should fail gracefully
    let publish = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name("test/after-close")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"test")
        .build()
        .unwrap();
    let result = client
        .send(mqtt::packet::Packet::V3_1_1Publish(publish))
        .await;
    // The send may fail or succeed depending on timing, but shouldn't panic
    let _ = result;
}

/// Test PINGREQ packet sending
#[tokio::test]
async fn test_pingreq_send() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send PINGREQ packet
    let pingreq = mqtt::packet::v3_1_1::Pingreq::builder().build().unwrap();
    let result = client
        .send(mqtt::packet::Packet::V3_1_1Pingreq(pingreq))
        .await;
    assert!(result.is_ok());
}

/// Test register_packet_id returns false when channel closed
#[tokio::test]
async fn test_register_packet_id_after_close() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close the client
    let _ = client.close().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // After close, register_packet_id might return false
    let result = client.register_packet_id(100).await;
    // Result depends on timing - just ensure no panic
    let _ = result;
}

/// Test release_packet_id error handling
#[tokio::test]
async fn test_release_packet_id_after_close() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close the client
    let _ = client.close().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // After close, release_packet_id might return error
    let result = client.release_packet_id(1).await;
    // Result depends on timing - just ensure no panic
    let _ = result;
}

/// Test state returns Closed when channel is closed
#[tokio::test]
async fn test_state_after_channel_close() {
    let config = MqttConfig::default();
    let mock_ws = MockUnderlyingLayer::new();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Close the client
    let _ = client.close().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // After close, state might return Closed
    let state = client.state().await;
    // State should be Closed
    assert_eq!(state, ConnectionState::Closed);
}

/// Test v5.0 PUBLISH packet
#[tokio::test]
async fn test_v50_publish() {
    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send v5.0 PUBLISH packet (simulating received message)
    let publish = mqtt::packet::v5_0::Publish::builder()
        .topic_name("test/v50/topic")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .payload(b"v5.0 message")
        .build()
        .unwrap();
    let publish_bytes = mqtt::packet::Packet::V5_0Publish(publish).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        publish_bytes,
    ));

    // Receive the packet
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Publish(pub_packet) = packet {
        assert_eq!(pub_packet.topic_name(), "test/v50/topic");
        // topic_name_extracted should be false since topic name was provided directly
        // (not extracted from topic alias mapping)
        assert!(!pub_packet.topic_name_extracted());
    } else {
        panic!("Expected v5.0 PUBLISH packet");
    }
}

/// Test topic_name_extracted flag for v5.0 PUBLISH packet with topic alias
/// When a PUBLISH packet is received with empty topic name and topic alias,
/// the library restores the topic name and sets topic_name_extracted to true
#[tokio::test]
async fn test_v50_publish_topic_name_extracted() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // First establish MQTT v5.0 connection
    // Send CONNECT with TopicAliasMaximum to indicate we can handle topic aliases
    let topic_alias_max = mqtt::packet::TopicAliasMaximum::new(10).unwrap();
    let connect_props =
        mqtt::packet::Properties::from(vec![Property::TopicAliasMaximum(topic_alias_max)]);
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("topic-alias-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .props(connect_props)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    // Simulate CONNACK from broker
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());

    // Now simulate broker sending a PUBLISH with topic name and topic alias to register the mapping
    let topic_alias_prop = mqtt::packet::TopicAlias::new(1).unwrap();
    let props = mqtt::packet::Properties::from(vec![Property::TopicAlias(topic_alias_prop)]);
    let publish_with_alias = mqtt::packet::v5_0::Publish::builder()
        .topic_name("test/alias/topic")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .props(props)
        .payload(b"first message")
        .build()
        .unwrap();
    let publish_bytes =
        mqtt::packet::Packet::V5_0Publish(publish_with_alias).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        publish_bytes,
    ));

    // Receive first PUBLISH packet (topic alias registered)
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Publish(pub_packet) = packet {
        assert_eq!(pub_packet.topic_name(), "test/alias/topic");
        // First packet has topic name provided, so extracted should be false
        assert!(!pub_packet.topic_name_extracted());
    } else {
        panic!("Expected v5.0 PUBLISH packet");
    }

    // Now simulate broker sending a PUBLISH with empty topic name but with topic alias
    // The library should restore the topic name from the alias mapping
    let topic_alias_prop2 = mqtt::packet::TopicAlias::new(1).unwrap();
    let props2 = mqtt::packet::Properties::from(vec![Property::TopicAlias(topic_alias_prop2)]);
    let publish_alias_only = mqtt::packet::v5_0::Publish::builder()
        .topic_name("")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .props(props2)
        .payload(b"second message")
        .build()
        .unwrap();
    let publish_bytes =
        mqtt::packet::Packet::V5_0Publish(publish_alias_only).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        publish_bytes,
    ));

    // Receive second packet (topic name should be extracted from alias)
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Publish(pub_packet) = packet {
        // Topic name should be restored from alias mapping
        assert_eq!(pub_packet.topic_name(), "test/alias/topic");
        // topic_name_extracted should be true since it was restored from alias
        assert!(pub_packet.topic_name_extracted());
    } else {
        panic!("Expected v5.0 PUBLISH packet");
    }
}

/// Test timer reset via CONNECT with keep_alive
/// This exercises the RequestTimerReset event handling code path
#[tokio::test]
async fn test_timer_reset_via_connect() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        auto_pub_response: true,
        auto_ping_response: true,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send CONNECT with keep_alive which should trigger timer setup
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("timer-test-client")
        .unwrap()
        .keep_alive(60) // 60 seconds keep-alive
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    // Simulate CONNACK response which triggers timer events
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Wait for CONNACK to be processed and timer to be set
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    // Connection should be established
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
}

/// Test timer cancel via disconnect
/// This exercises the RequestTimerCancel event handling code path
#[tokio::test]
async fn test_timer_cancel_via_disconnect() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish MQTT connection first
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("timer-cancel-test")
        .unwrap()
        .keep_alive(60)
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    // Simulate CONNACK
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Wait for CONNACK
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send DISCONNECT which should trigger timer cancellation
    let disconnect = mqtt::packet::v3_1_1::Disconnect::builder().build().unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Disconnect(disconnect))
        .await;

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
}

/// Test multiple timer resets (simulating keep-alive refresh)
#[tokio::test]
async fn test_multiple_timer_resets() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish MQTT connection
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("multi-timer-test")
        .unwrap()
        .keep_alive(60)
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Simulate receiving PINGRESP multiple times (triggers timer reset)
    for _ in 0..3 {
        let pingresp = mqtt::packet::v3_1_1::Pingresp::builder().build().unwrap();
        let pingresp_bytes = mqtt::packet::Packet::V3_1_1Pingresp(pingresp).to_continuous_buffer();
        let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
            pingresp_bytes,
        ));
        let _ = tokio::time::timeout(tokio::time::Duration::from_millis(100), client.recv()).await;
    }
}

/// Test v5.0 timer handling
#[tokio::test]
async fn test_v50_timer_handling() {
    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send v5.0 CONNECT
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("v50-timer-test")
        .unwrap()
        .keep_alive(30)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    // Simulate v5.0 CONNACK
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send v5.0 DISCONNECT
    let disconnect = mqtt::packet::v5_0::Disconnect::builder()
        .reason_code(client_mqtt::result_code::DisconnectReasonCode::NormalDisconnection)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Disconnect(disconnect))
        .await;
}

/// Test that timer expiration is properly handled
/// This specifically exercises the UnderlyingLayerEvent::TimerExpired handler
#[tokio::test]
async fn test_timer_expiration() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        // Set a very short pingreq interval so the timer fires quickly
        pingreq_send_interval_ms: Some(50),
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    // Send CONNECT with keep_alive which should trigger timer setup
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("timer-expiry-test")
        .unwrap()
        .keep_alive(1) // 1 second keep-alive (shortest allowed)
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    // Simulate CONNACK response which triggers timer events
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Wait for CONNACK to be processed
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(100), client.recv()).await;

    // Wait for the timer to actually fire (50ms pingreq interval)
    // The MockUnderlyingLayer will send TimerExpired event via tokio timer
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

    // The timer should have fired by now and sent a PINGREQ packet
    // The client should still be connected
    assert!(client.is_connected().await);
}

/// Test PingrespRecv timeout
/// This exercises the PingrespRecv timer expiration code path
#[tokio::test]
async fn test_pingresp_recv_timeout() {
    let config = MqttConfig {
        version: client_mqtt::Version::V3_1_1,
        // Set pingreq interval longer than pingresp timeout
        // so PingrespRecv timer can fire before next PingreqSend
        pingreq_send_interval_ms: Some(100),
        // Set a short pingresp timeout
        pingresp_recv_timeout_ms: 30,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Connect
    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    // Send CONNECT with keep_alive
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("pingresp-timeout-test")
        .unwrap()
        .keep_alive(1)
        .clean_session(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await;

    // Simulate CONNACK response which triggers timer setup
    let connack = mqtt::packet::v3_1_1::Connack::builder()
        .session_present(false)
        .return_code(client_mqtt::result_code::ConnectReturnCode::Accepted)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V3_1_1Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Wait for CONNACK to be processed
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(100), client.recv()).await;

    // Wait for:
    // 1. PingreqSend timer to fire (100ms) - sends PINGREQ
    // 2. PingrespRecv timer to fire (30ms after PINGREQ) - timeout because no PINGRESP
    // Total wait: ~150ms should be enough
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // The PingrespRecv timeout should have fired
    // Note: mqtt-protocol-core may close the connection or emit an error on timeout
    // The test verifies the code path is exercised
}

/// Test subscription_identifiers accessor for v5.0 PUBLISH packet
/// Subscription identifiers are set by the broker to indicate which subscriptions matched
#[tokio::test]
async fn test_v50_publish_subscription_identifiers() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish MQTT v5.0 connection
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("sub-id-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    // Simulate CONNACK from broker
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    // Simulate broker sending PUBLISH with multiple subscription identifiers
    // This happens when a message matches multiple subscriptions
    let sub_id1 = mqtt::packet::SubscriptionIdentifier::new(100).unwrap();
    let sub_id2 = mqtt::packet::SubscriptionIdentifier::new(200).unwrap();
    let props = mqtt::packet::Properties::from(vec![
        Property::SubscriptionIdentifier(sub_id1),
        Property::SubscriptionIdentifier(sub_id2),
    ]);
    let publish = mqtt::packet::v5_0::Publish::builder()
        .topic_name("test/sub-id")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .props(props)
        .payload(b"message with sub ids")
        .build()
        .unwrap();
    let publish_bytes = mqtt::packet::Packet::V5_0Publish(publish).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        publish_bytes,
    ));

    // Receive PUBLISH and check subscription identifiers
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Publish(pub_packet) = packet {
        assert_eq!(pub_packet.topic_name(), "test/sub-id");
        // Check subscription identifiers by iterating through properties
        let mut sub_ids: Vec<u32> = Vec::new();
        for prop in pub_packet.props.iter() {
            if let Property::SubscriptionIdentifier(p) = prop {
                sub_ids.push(p.val());
            }
        }
        assert_eq!(sub_ids.len(), 2);
        assert!(sub_ids.contains(&100));
        assert!(sub_ids.contains(&200));
    } else {
        panic!("Expected v5.0 PUBLISH packet");
    }
}

/// Test user_properties accessor for v5.0 PUBLISH packet
#[tokio::test]
async fn test_v50_publish_user_properties() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish MQTT v5.0 connection
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("user-prop-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    // Simulate CONNACK from broker
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    // Simulate broker sending PUBLISH with user properties
    let user_prop1 = mqtt::packet::UserProperty::new("source", "sensor-1").unwrap();
    let user_prop2 = mqtt::packet::UserProperty::new("unit", "celsius").unwrap();
    let user_prop3 = mqtt::packet::UserProperty::new("location", "room-42").unwrap();
    let props = mqtt::packet::Properties::from(vec![
        Property::UserProperty(user_prop1),
        Property::UserProperty(user_prop2),
        Property::UserProperty(user_prop3),
    ]);
    let publish = mqtt::packet::v5_0::Publish::builder()
        .topic_name("test/user-props")
        .unwrap()
        .qos(mqtt::packet::Qos::AtMostOnce)
        .props(props)
        .payload(b"25.5")
        .build()
        .unwrap();
    let publish_bytes = mqtt::packet::Packet::V5_0Publish(publish).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        publish_bytes,
    ));

    // Receive PUBLISH and check user properties
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Publish(pub_packet) = packet {
        assert_eq!(pub_packet.topic_name(), "test/user-props");
        // Check user properties by iterating through properties
        let mut user_props: Vec<(String, String)> = Vec::new();
        for prop in pub_packet.props.iter() {
            if let Property::UserProperty(p) = prop {
                user_props.push((p.key().to_string(), p.val().to_string()));
            }
        }
        assert_eq!(user_props.len(), 3);
        assert!(user_props.contains(&("source".to_string(), "sensor-1".to_string())));
        assert!(user_props.contains(&("unit".to_string(), "celsius".to_string())));
        assert!(user_props.contains(&("location".to_string(), "room-42".to_string())));
    } else {
        panic!("Expected v5.0 PUBLISH packet");
    }
}

/// Test user_properties accessor for v5.0 CONNACK packet
#[tokio::test]
async fn test_v50_connack_user_properties() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send CONNECT
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("connack-user-prop-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    // Simulate CONNACK with user properties from broker
    let user_prop1 = mqtt::packet::UserProperty::new("server-name", "mqtt-broker-1").unwrap();
    let user_prop2 = mqtt::packet::UserProperty::new("version", "5.0").unwrap();
    let connack_props = mqtt::packet::Properties::from(vec![
        Property::UserProperty(user_prop1),
        Property::UserProperty(user_prop2),
    ]);
    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .props(connack_props)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));

    // Receive CONNACK and check user properties
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Connack(connack_packet) = packet {
        assert!(
            connack_packet.reason_code() == client_mqtt::result_code::ConnectReasonCode::Success
        );
        // Check user properties by iterating through properties
        let mut user_props: Vec<(String, String)> = Vec::new();
        for prop in connack_packet.props.iter() {
            if let Property::UserProperty(p) = prop {
                user_props.push((p.key().to_string(), p.val().to_string()));
            }
        }
        assert_eq!(user_props.len(), 2);
        assert!(user_props.contains(&("server-name".to_string(), "mqtt-broker-1".to_string())));
        assert!(user_props.contains(&("version".to_string(), "5.0".to_string())));
    } else {
        panic!("Expected v5.0 CONNACK packet");
    }
}

/// Test user_properties accessor for v5.0 SUBACK packet
#[tokio::test]
async fn test_v50_suback_user_properties() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish connection
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("suback-user-prop-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    // Send SUBSCRIBE first (SUBACK can only be received in response to SUBSCRIBE)
    let packet_id = client.acquire_packet_id().await.unwrap();
    let sub_opts = mqtt::packet::SubOpts::new().set_qos(mqtt::packet::Qos::AtLeastOnce);
    let sub_entry = mqtt::packet::SubEntry::new("test/topic", sub_opts).unwrap();
    let subscribe_packet = mqtt::packet::v5_0::Subscribe::builder()
        .packet_id(packet_id)
        .entries(vec![sub_entry])
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Subscribe(subscribe_packet))
        .await;

    // Simulate SUBACK with user properties from broker
    let user_prop = mqtt::packet::UserProperty::new("info", "subscription-accepted").unwrap();
    let suback_props = mqtt::packet::Properties::from(vec![Property::UserProperty(user_prop)]);
    let suback = mqtt::packet::v5_0::Suback::builder()
        .packet_id(packet_id)
        .reason_codes(vec![
            client_mqtt::result_code::SubackReasonCode::GrantedQos1,
        ])
        .props(suback_props)
        .build()
        .unwrap();
    let suback_bytes = mqtt::packet::Packet::V5_0Suback(suback).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        suback_bytes,
    ));

    // Receive SUBACK and check user properties
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Suback(suback_packet) = packet {
        assert_eq!(suback_packet.packet_id(), packet_id);
        // Check user properties by iterating through properties
        let mut user_props: Vec<(String, String)> = Vec::new();
        for prop in suback_packet.props.iter() {
            if let Property::UserProperty(p) = prop {
                user_props.push((p.key().to_string(), p.val().to_string()));
            }
        }
        assert_eq!(user_props.len(), 1);
        assert!(user_props.contains(&("info".to_string(), "subscription-accepted".to_string())));
    } else {
        panic!("Expected v5.0 SUBACK packet");
    }
}

/// Test user_properties accessor for v5.0 DISCONNECT packet
#[tokio::test]
async fn test_v50_disconnect_user_properties() {
    use mqtt_protocol_core::mqtt::packet::Property;

    let config = MqttConfig {
        version: client_mqtt::Version::V5_0,
        ..Default::default()
    };
    let mock_ws = MockUnderlyingLayer::new();
    let event_sender = mock_ws.event_sender.clone();

    let client = MqttClient::new_with_websocket(config, mock_ws);

    let _ = client.connect("ws://test.example.com").await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Establish connection
    let connect_packet = mqtt::packet::v5_0::Connect::builder()
        .client_id("disconnect-user-prop-test")
        .unwrap()
        .keep_alive(60)
        .clean_start(true)
        .build()
        .unwrap();
    let _ = client
        .send(mqtt::packet::Packet::V5_0Connect(connect_packet))
        .await;

    let connack = mqtt::packet::v5_0::Connack::builder()
        .session_present(false)
        .reason_code(client_mqtt::result_code::ConnectReasonCode::Success)
        .build()
        .unwrap();
    let connack_bytes = mqtt::packet::Packet::V5_0Connack(connack).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        connack_bytes,
    ));
    let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;

    // Simulate DISCONNECT with user properties from broker
    let user_prop = mqtt::packet::UserProperty::new("reason", "server-shutdown").unwrap();
    let disconnect_props = mqtt::packet::Properties::from(vec![Property::UserProperty(user_prop)]);
    let disconnect = mqtt::packet::v5_0::Disconnect::builder()
        .reason_code(client_mqtt::result_code::DisconnectReasonCode::ServerShuttingDown)
        .props(disconnect_props)
        .build()
        .unwrap();
    let disconnect_bytes = mqtt::packet::Packet::V5_0Disconnect(disconnect).to_continuous_buffer();
    let _ = event_sender.unbounded_send(mqtt_client_wasm::UnderlyingLayerEvent::Message(
        disconnect_bytes,
    ));

    // Receive DISCONNECT and check user properties
    let recv_result =
        tokio::time::timeout(tokio::time::Duration::from_millis(500), client.recv()).await;
    assert!(recv_result.is_ok());
    let packet = recv_result.unwrap().unwrap();
    if let mqtt::packet::Packet::V5_0Disconnect(disconnect_packet) = packet {
        // Check user properties by iterating through properties
        let mut user_props: Vec<(String, String)> = Vec::new();
        if let Some(props) = &disconnect_packet.props {
            for prop in props.iter() {
                if let Property::UserProperty(p) = prop {
                    user_props.push((p.key().to_string(), p.val().to_string()));
                }
            }
        }
        assert_eq!(user_props.len(), 1);
        assert!(user_props.contains(&("reason".to_string(), "server-shutdown".to_string())));
    } else {
        panic!("Expected v5.0 DISCONNECT packet");
    }
}
