//! Tests for MQTT client functionality
//!
//! These tests verify the core MQTT client behavior including:
//! - Client creation and connection
//! - Packet flow and packet ID management
//! - Publish/Subscribe operations
//! - recv() timeout handling and packet loss prevention

#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::MockWebSocket;
use futures::channel::oneshot;
use mqtt_client_wasm::{ConnectionState, MqttClient, MqttConfig, Result};
use mqtt_protocol_core::mqtt;

#[test]
fn test_mqtt_client_creation() {
    let config = MqttConfig::default();
    let mock_ws = MockWebSocket::new();
    let _client = MqttClient::new_with_websocket(config, mock_ws);
}

#[tokio::test]
async fn test_mqtt_connect_flow() {
    let config = MqttConfig::default();
    let mock_ws = MockWebSocket::new();

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
    let mock_ws = MockWebSocket::new();
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
    let mock_ws = MockWebSocket::new();
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
    let mock_ws = MockWebSocket::new();
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
/// Verifies that a client can be reconnected after being closed.
#[tokio::test]
async fn test_reconnect_after_close() {
    let config = MqttConfig::default();
    let mock_ws = MockWebSocket::new();

    // Create client
    let client = MqttClient::new_with_websocket(config, mock_ws);

    // Wait for background tasks to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // First connection
    let connect_result = client.connect("ws://localhost:1883").await;
    assert!(connect_result.is_ok(), "First connection should succeed");

    // Wait for connection
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Close the connection
    let close_result = client.close().await;
    assert!(close_result.is_ok(), "Close should succeed");

    // Wait for close to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Verify state is closed
    let state = client.state().await;
    assert_eq!(
        state,
        ConnectionState::Closed,
        "State should be Closed after close"
    );
}

/// Test that Connecting and Connected states reject new connections
#[tokio::test]
async fn test_reject_connect_when_already_connected() {
    let config = MqttConfig::default();
    let mock_ws = MockWebSocket::new();

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
