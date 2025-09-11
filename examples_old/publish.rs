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
    let payload = "Hello from WASM!";

    web_sys::console::log_1(&format!("Simple MQTT Publisher").into());
    web_sys::console::log_1(&format!("Broker: {}:{}", hostname, port).into());
    web_sys::console::log_1(&format!("Topic: {}", topic).into());
    web_sys::console::log_1(&format!("QoS: {}", qos).into());
    web_sys::console::log_1(&format!("Payload: {}", payload).into());

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
        .client_id("wasm_publisher")
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

    // Create PUBLISH packet
    let publish_builder = mqtt::packet::v3_1_1::Publish::builder()
        .topic_name(topic)
        .unwrap()
        .qos(qos_level)
        .retain(false)
        .payload(payload.as_bytes());

    // Add packet ID for QoS 1 and 2 using proper packet ID management
    let acquired_packet_id = if qos_level != mqtt::packet::Qos::AtMostOnce {
        match client.acquire_packet_id() {
            Some(id) => {
                web_sys::console::log_1(&format!("Acquired packet ID: {}", id).into());
                Some(id)
            }
            None => {
                web_sys::console::error_1(&"Failed to acquire packet ID".into());
                return;
            }
        }
    } else {
        None
    };

    let publish_packet = if let Some(packet_id) = acquired_packet_id {
        publish_builder.packet_id(packet_id).build().unwrap()
    } else {
        publish_builder.build().unwrap()
    };

    web_sys::console::log_1(&format!("Publishing message...").into());
    if let Err(e) = client
        .send(mqtt::packet::Packet::V3_1_1Publish(publish_packet.clone()))
        .await
    {
        web_sys::console::error_1(&format!("Failed to send PUBLISH: {:?}", e).into());
        return;
    }

    // Handle QoS acknowledgements
    match qos_level {
        mqtt::packet::Qos::AtMostOnce => {
            web_sys::console::log_1(
                &"Message published successfully (QoS 0 - fire and forget)".into(),
            );
        }
        mqtt::packet::Qos::AtLeastOnce => {
            // Wait for PUBACK
            web_sys::console::log_1(&"Waiting for PUBACK...".into());
            match client.recv().await {
                Ok(packet) => match packet {
                    mqtt::packet::Packet::V3_1_1Puback(puback) => {
                        web_sys::console::log_1(&format!("Message published successfully (QoS 1 - received PUBACK for packet ID {})", puback.packet_id()).into());

                        // Release the packet ID now that QoS 1 flow is complete
                        if let Some(packet_id) = acquired_packet_id {
                            if let Err(e) = client.release_packet_id(packet_id) {
                                web_sys::console::error_1(
                                    &format!("Failed to release packet ID {}: {:?}", packet_id, e)
                                        .into(),
                                );
                            } else {
                                web_sys::console::log_1(
                                    &format!("Released packet ID: {}", packet_id).into(),
                                );
                            }
                        }
                    }
                    _ => {
                        web_sys::console::error_1(
                            &format!("Expected PUBACK, received: {:?}", packet.packet_type())
                                .into(),
                        );
                        return;
                    }
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to receive PUBACK: {:?}", e).into());
                    return;
                }
            }
        }
        mqtt::packet::Qos::ExactlyOnce => {
            // Wait for PUBREC
            web_sys::console::log_1(&"Waiting for PUBREC...".into());
            match client.recv().await {
                Ok(packet) => {
                    match packet {
                        mqtt::packet::Packet::V3_1_1Pubrec(pubrec) => {
                            web_sys::console::log_1(
                                &format!("Received PUBREC for packet ID {}", pubrec.packet_id())
                                    .into(),
                            );

                            // Send PUBREL
                            let pubrel = mqtt::packet::v3_1_1::Pubrel::builder()
                                .packet_id(pubrec.packet_id())
                                .build()
                                .unwrap();
                            if let Err(e) = client
                                .send(mqtt::packet::Packet::V3_1_1Pubrel(pubrel))
                                .await
                            {
                                web_sys::console::error_1(
                                    &format!("Failed to send PUBREL: {:?}", e).into(),
                                );
                                return;
                            }

                            // Wait for PUBCOMP
                            web_sys::console::log_1(&"Waiting for PUBCOMP...".into());
                            match client.recv().await {
                                Ok(packet) => match packet {
                                    mqtt::packet::Packet::V3_1_1Pubcomp(pubcomp) => {
                                        web_sys::console::log_1(&format!("Message published successfully (QoS 2 - received PUBCOMP for packet ID {})", pubcomp.packet_id()).into());

                                        // Release the packet ID now that QoS 2 flow is complete
                                        if let Some(packet_id) = acquired_packet_id {
                                            if let Err(e) = client.release_packet_id(packet_id) {
                                                web_sys::console::error_1(
                                                    &format!(
                                                        "Failed to release packet ID {}: {:?}",
                                                        packet_id, e
                                                    )
                                                    .into(),
                                                );
                                            } else {
                                                web_sys::console::log_1(
                                                    &format!("Released packet ID: {}", packet_id)
                                                        .into(),
                                                );
                                            }
                                        }
                                    }
                                    _ => {
                                        web_sys::console::error_1(
                                            &format!(
                                                "Expected PUBCOMP, received: {:?}",
                                                packet.packet_type()
                                            )
                                            .into(),
                                        );
                                        return;
                                    }
                                },
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Failed to receive PUBCOMP: {:?}", e).into(),
                                    );
                                    return;
                                }
                            }
                        }
                        _ => {
                            web_sys::console::error_1(
                                &format!("Expected PUBREC, received: {:?}", packet.packet_type())
                                    .into(),
                            );
                            return;
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to receive PUBREC: {:?}", e).into());
                    return;
                }
            }
        }
    }

    // Send DISCONNECT packet
    web_sys::console::log_1(&"Sending DISCONNECT packet...".into());
    let disconnect_packet = mqtt::packet::v3_1_1::Disconnect::new();
    if let Err(e) = client
        .send(mqtt::packet::Packet::V3_1_1Disconnect(disconnect_packet))
        .await
    {
        web_sys::console::log_1(&format!("Warning: Failed to send DISCONNECT: {:?}", e).into());
    }

    // Close connection
    if let Err(e) = client.close().await {
        web_sys::console::log_1(&format!("Warning: Failed to close connection: {:?}", e).into());
    }

    web_sys::console::log_1(&"Publisher finished successfully".into());
}
