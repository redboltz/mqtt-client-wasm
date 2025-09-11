#![no_main]
use mqtt_client_wasm::{mqtt, MqttClient, MqttConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn main() {
    spawn_local(async_main());
}

async fn async_main() {
    console_error_panic_hook::set_once();

    let hostname = "mqtt.redboltz.net";
    let port = 10080;
    let topic = "wasm/test";
    let qos = 1u8;

    web_sys::console::log_1(&format!("Simple MQTT Subscriber").into());
    web_sys::console::log_1(&format!("Broker: {}:{}", hostname, port).into());
    web_sys::console::log_1(&format!("Topic: {}", topic).into());
    web_sys::console::log_1(&format!("QoS: {}", qos).into());

    // Create configuration
    let config = MqttConfig {
        url: format!("ws://{}:{}/mqtt", hostname, port),
        version: mqtt::Version::V3_1_1,
        ..Default::default()
    };

    // Create client
    let mut client = MqttClient::new(config);

    // Connect to broker
    web_sys::console::log_1(&"Connecting to broker...".into());
    match client.connect().await {
        Ok(_) => web_sys::console::log_1(&"Connected to broker".into()),
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to connect: {:?}", e).into());
            return;
        }
    }

    // Send MQTT CONNECT packet
    web_sys::console::log_1(&"Sending CONNECT packet...".into());
    let connect_packet = mqtt::packet::v3_1_1::Connect::builder()
        .client_id("wasm_subscriber")
        .unwrap()
        .keep_alive(60)
        .clean_session(true)
        .build()
        .unwrap();

    if let Err(e) = client
        .send(mqtt::packet::Packet::V3_1_1Connect(connect_packet))
        .await
    {
        web_sys::console::error_1(&format!("Failed to send CONNECT: {:?}", e).into());
        return;
    }

    // Wait for CONNACK
    web_sys::console::log_1(&"Waiting for CONNACK...".into());
    match client.recv().await {
        Ok(packet) => match packet {
            mqtt::packet::Packet::V3_1_1Connack(connack) => {
                if connack.return_code() != mqtt::result_code::ConnectReturnCode::Accepted {
                    web_sys::console::error_1(
                        &format!("Connection refused: {:?}", connack.return_code()).into(),
                    );
                    return;
                }
                web_sys::console::log_1(&"MQTT connection accepted by broker".into());
            }
            _ => {
                web_sys::console::error_1(
                    &format!("Expected CONNACK, received: {:?}", packet.packet_type()).into(),
                );
                return;
            }
        },
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to receive CONNACK: {:?}", e).into());
            return;
        }
    }

    // Convert QoS
    let qos_level =
        mqtt::packet::Qos::try_from(qos).expect("Invalid QoS level. Must be 0, 1, or 2");

    // Create SubEntry for the subscription
    let sub_opts = mqtt::packet::SubOpts::new().set_qos(qos_level);
    let sub_entry = match mqtt::packet::SubEntry::new(topic, sub_opts) {
        Ok(entry) => entry,
        Err(e) => {
            web_sys::console::error_1(
                &format!("Failed to create subscription entry: {:?}", e).into(),
            );
            return;
        }
    };

    // Create SUBSCRIBE packet with proper packet ID management
    web_sys::console::log_1(&format!("Sending SUBSCRIBE packet for topic '{}'...", topic).into());
    let packet_id = match client.acquire_packet_id() {
        Some(id) => {
            web_sys::console::log_1(&format!("Acquired packet ID: {}", id).into());
            id
        }
        None => {
            web_sys::console::error_1(&"Failed to acquire packet ID for SUBSCRIBE".into());
            return;
        }
    };

    let subscribe_packet = mqtt::packet::v3_1_1::Subscribe::builder()
        .packet_id(packet_id)
        .entries(vec![sub_entry])
        .build()
        .unwrap();

    if let Err(e) = client
        .send(mqtt::packet::Packet::V3_1_1Subscribe(subscribe_packet))
        .await
    {
        web_sys::console::error_1(&format!("Failed to send SUBSCRIBE: {:?}", e).into());
        return;
    }

    // Wait for SUBACK
    web_sys::console::log_1(&"Waiting for SUBACK...".into());
    match client.recv().await {
        Ok(packet) => match packet {
            mqtt::packet::Packet::V3_1_1Suback(suback) => {
                web_sys::console::log_1(
                    &format!("Subscription confirmed for topic: {}", topic).into(),
                );
                web_sys::console::log_1(
                    &format!("Granted QoS levels: {:?}", suback.return_codes()).into(),
                );

                // Release the packet ID now that subscription is confirmed
                if let Err(e) = client.release_packet_id(packet_id) {
                    web_sys::console::error_1(
                        &format!("Failed to release packet ID {}: {:?}", packet_id, e).into(),
                    );
                } else {
                    web_sys::console::log_1(&format!("Released packet ID: {}", packet_id).into());
                }
            }
            _ => {
                web_sys::console::error_1(
                    &format!("Expected SUBACK, received: {:?}", packet.packet_type()).into(),
                );
                return;
            }
        },
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to receive SUBACK: {:?}", e).into());
            return;
        }
    }

    web_sys::console::log_1(
        &format!(
            "Successfully subscribed to topic '{}' with QoS {:?}",
            topic, qos_level
        )
        .into(),
    );
    web_sys::console::log_1(&"Listening for messages...".into());

    // Message receive loop
    loop {
        match client.recv().await {
            Ok(packet) => {
                match packet {
                    mqtt::packet::Packet::V3_1_1Publish(publish) => {
                        web_sys::console::log_1(&"Received message:".into());
                        web_sys::console::log_1(
                            &format!("  Topic: {}", publish.topic_name()).into(),
                        );
                        web_sys::console::log_1(&format!("  QoS: {:?}", publish.qos()).into());
                        web_sys::console::log_1(&format!("  Retain: {}", publish.retain()).into());

                        // Convert payload to string for display
                        let payload_str = match std::str::from_utf8(publish.payload().as_slice()) {
                            Ok(s) => s.to_string(),
                            Err(_) => format!("<binary data: {} bytes>", publish.payload().len()),
                        };
                        web_sys::console::log_1(&format!("  Payload: {}", payload_str).into());
                        web_sys::console::log_1(&"  ---".into());

                        // Send PUBACK if QoS 1
                        if publish.qos() == mqtt::packet::Qos::AtLeastOnce {
                            if let Some(packet_id) = publish.packet_id() {
                                let puback = mqtt::packet::v3_1_1::Puback::builder()
                                    .packet_id(packet_id)
                                    .build()
                                    .unwrap();
                                if let Err(e) = client
                                    .send(mqtt::packet::Packet::V3_1_1Puback(puback))
                                    .await
                                {
                                    web_sys::console::error_1(
                                        &format!("Failed to send PUBACK: {:?}", e).into(),
                                    );
                                }
                            }
                        }
                        // Handle QoS 2 PUBREC/PUBREL/PUBCOMP flow if needed
                        else if publish.qos() == mqtt::packet::Qos::ExactlyOnce {
                            if let Some(packet_id) = publish.packet_id() {
                                let pubrec = mqtt::packet::v3_1_1::Pubrec::builder()
                                    .packet_id(packet_id)
                                    .build()
                                    .unwrap();
                                if let Err(e) = client
                                    .send(mqtt::packet::Packet::V3_1_1Pubrec(pubrec))
                                    .await
                                {
                                    web_sys::console::error_1(
                                        &format!("Failed to send PUBREC: {:?}", e).into(),
                                    );
                                }
                            }
                        }
                    }
                    mqtt::packet::Packet::V3_1_1Pubrel(pubrel) => {
                        // Send PUBCOMP for QoS 2 flow
                        let pubcomp = mqtt::packet::v3_1_1::Pubcomp::builder()
                            .packet_id(pubrel.packet_id())
                            .build()
                            .unwrap();
                        if let Err(e) = client
                            .send(mqtt::packet::Packet::V3_1_1Pubcomp(pubcomp))
                            .await
                        {
                            web_sys::console::error_1(
                                &format!("Failed to send PUBCOMP: {:?}", e).into(),
                            );
                        }
                    }
                    _ => {
                        // Other packet types
                        web_sys::console::log_1(
                            &format!("Received packet: {:?}", packet.packet_type()).into(),
                        );
                    }
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to receive packet: {:?}", e).into());
                break;
            }
        }
    }

    // Close connection
    if let Err(e) = client.close().await {
        web_sys::console::log_1(&format!("Warning: Failed to close connection: {:?}", e).into());
    }

    web_sys::console::log_1(&"Subscriber stopped".into());
}
