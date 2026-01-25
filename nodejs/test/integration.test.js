/**
 * Integration tests for Node.js MQTT client
 *
 * Tests TCP, TLS, WebSocket, and WSS connections with MQTT v3.1.1 and v5.0
 */

const assert = require('assert');
const fs = require('fs');
const { startBroker, stopBroker, getConnectionInfo } = require('./broker-helper');
const {
    WasmMqttClient,
    WasmMqttConfig,
    WasmMqttPacket,
    WasmPacketType,
    init,
    NodeWebSocketTransport,
    NodeTcpTransport,
    NodeTlsTransport
} = require('../index.js');

// Initialize WASM
init();

// Test timeout
const TEST_TIMEOUT = 10000;

/**
 * Helper to wait for a specific packet type
 */
async function waitForPacket(transport, expectedType, timeout = 5000) {
    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            reject(new Error(`Timeout waiting for packet type: ${expectedType}`));
        }, timeout);

        transport.onMessage((data) => {
            try {
                const packet = WasmMqttPacket.fromBytes(data, 'v5.0');
                const packetType = packet.packetType;
                if (packetType === expectedType) {
                    clearTimeout(timer);
                    resolve(packet);
                }
            } catch (e) {
                // Continue waiting
            }
        });
    });
}

/**
 * Create CONNECT packet bytes
 */
function createConnectPacket(version, clientId, cleanSession = true) {
    const config = new WasmMqttConfig({ version });
    const client = new WasmMqttClient(config);

    // Build CONNECT packet using the WASM client
    const connectOptions = {
        clientId: clientId,
        cleanSession: cleanSession,
        keepAlive: 60
    };

    // Get CONNECT packet bytes from client
    // For now, manually construct a simple CONNECT packet
    if (version === 'v3.1.1') {
        return createConnectPacketV311(clientId, cleanSession);
    } else {
        return createConnectPacketV50(clientId, cleanSession);
    }
}

/**
 * Create MQTT v3.1.1 CONNECT packet
 */
function createConnectPacketV311(clientId, cleanSession) {
    const protocolName = Buffer.from('MQTT');
    const protocolLevel = 4; // v3.1.1
    const connectFlags = cleanSession ? 0x02 : 0x00;
    const keepAlive = 60;

    const clientIdBuf = Buffer.from(clientId, 'utf8');

    // Variable header
    const variableHeader = Buffer.alloc(10);
    variableHeader.writeUInt16BE(4, 0); // Protocol name length
    protocolName.copy(variableHeader, 2);
    variableHeader.writeUInt8(protocolLevel, 6);
    variableHeader.writeUInt8(connectFlags, 7);
    variableHeader.writeUInt16BE(keepAlive, 8);

    // Payload
    const payload = Buffer.alloc(2 + clientIdBuf.length);
    payload.writeUInt16BE(clientIdBuf.length, 0);
    clientIdBuf.copy(payload, 2);

    // Fixed header
    const remainingLength = variableHeader.length + payload.length;
    const fixedHeader = Buffer.alloc(2);
    fixedHeader.writeUInt8(0x10, 0); // CONNECT packet type
    fixedHeader.writeUInt8(remainingLength, 1);

    return Buffer.concat([fixedHeader, variableHeader, payload]);
}

/**
 * Create MQTT v5.0 CONNECT packet
 */
function createConnectPacketV50(clientId, cleanSession) {
    const protocolName = Buffer.from('MQTT');
    const protocolLevel = 5; // v5.0
    const connectFlags = cleanSession ? 0x02 : 0x00;
    const keepAlive = 60;

    const clientIdBuf = Buffer.from(clientId, 'utf8');

    // Variable header (with properties length = 0)
    const variableHeader = Buffer.alloc(11);
    variableHeader.writeUInt16BE(4, 0); // Protocol name length
    protocolName.copy(variableHeader, 2);
    variableHeader.writeUInt8(protocolLevel, 6);
    variableHeader.writeUInt8(connectFlags, 7);
    variableHeader.writeUInt16BE(keepAlive, 8);
    variableHeader.writeUInt8(0, 10); // Properties length = 0

    // Payload
    const payload = Buffer.alloc(2 + clientIdBuf.length);
    payload.writeUInt16BE(clientIdBuf.length, 0);
    clientIdBuf.copy(payload, 2);

    // Fixed header
    const remainingLength = variableHeader.length + payload.length;
    const fixedHeader = Buffer.alloc(2);
    fixedHeader.writeUInt8(0x10, 0); // CONNECT packet type
    fixedHeader.writeUInt8(remainingLength, 1);

    return Buffer.concat([fixedHeader, variableHeader, payload]);
}

/**
 * Create DISCONNECT packet
 */
function createDisconnectPacket(version) {
    if (version === 'v3.1.1') {
        return Buffer.from([0xE0, 0x00]);
    } else {
        // v5.0 with reason code 0 and no properties
        return Buffer.from([0xE0, 0x02, 0x00, 0x00]);
    }
}

/**
 * Create SUBSCRIBE packet
 */
function createSubscribePacket(version, packetId, topic, qos = 0) {
    const topicBuf = Buffer.from(topic, 'utf8');

    if (version === 'v3.1.1') {
        const variableHeader = Buffer.alloc(2);
        variableHeader.writeUInt16BE(packetId, 0);

        const payload = Buffer.alloc(2 + topicBuf.length + 1);
        payload.writeUInt16BE(topicBuf.length, 0);
        topicBuf.copy(payload, 2);
        payload.writeUInt8(qos, 2 + topicBuf.length);

        const remainingLength = variableHeader.length + payload.length;
        const fixedHeader = Buffer.from([0x82, remainingLength]);

        return Buffer.concat([fixedHeader, variableHeader, payload]);
    } else {
        // v5.0 with properties length = 0
        const variableHeader = Buffer.alloc(3);
        variableHeader.writeUInt16BE(packetId, 0);
        variableHeader.writeUInt8(0, 2); // Properties length = 0

        const payload = Buffer.alloc(2 + topicBuf.length + 1);
        payload.writeUInt16BE(topicBuf.length, 0);
        topicBuf.copy(payload, 2);
        payload.writeUInt8(qos, 2 + topicBuf.length);

        const remainingLength = variableHeader.length + payload.length;
        const fixedHeader = Buffer.from([0x82, remainingLength]);

        return Buffer.concat([fixedHeader, variableHeader, payload]);
    }
}

/**
 * Create PUBLISH packet
 */
function createPublishPacket(version, topic, payload, qos = 0, packetId = null) {
    const topicBuf = Buffer.from(topic, 'utf8');
    const payloadBuf = Buffer.from(payload, 'utf8');

    const topicLength = Buffer.alloc(2);
    topicLength.writeUInt16BE(topicBuf.length, 0);

    let variableHeader;
    if (qos > 0 && packetId !== null) {
        const packetIdBuf = Buffer.alloc(2);
        packetIdBuf.writeUInt16BE(packetId, 0);
        if (version === 'v5.0') {
            variableHeader = Buffer.concat([topicLength, topicBuf, packetIdBuf, Buffer.from([0x00])]); // Properties length = 0
        } else {
            variableHeader = Buffer.concat([topicLength, topicBuf, packetIdBuf]);
        }
    } else {
        if (version === 'v5.0') {
            variableHeader = Buffer.concat([topicLength, topicBuf, Buffer.from([0x00])]); // Properties length = 0
        } else {
            variableHeader = Buffer.concat([topicLength, topicBuf]);
        }
    }

    const remainingLength = variableHeader.length + payloadBuf.length;
    const fixedHeaderByte = 0x30 | (qos << 1);
    const fixedHeader = Buffer.from([fixedHeaderByte, remainingLength]);

    return Buffer.concat([fixedHeader, variableHeader, payloadBuf]);
}

/**
 * Run a basic connect/disconnect test
 */
async function runConnectTest(transport, version, testName) {
    const clientId = `test-${version}-${Date.now()}`;

    return new Promise((resolve, reject) => {
        const timeout = setTimeout(() => {
            reject(new Error(`${testName}: Timeout`));
        }, TEST_TIMEOUT);

        transport.onMessage((data) => {
            try {
                // Check for CONNACK (packet type 0x20)
                if (data[0] === 0x20) {
                    console.log(`  ${testName}: Received CONNACK`);

                    // Send DISCONNECT
                    transport.send(createDisconnectPacket(version));
                    console.log(`  ${testName}: Sent DISCONNECT`);

                    clearTimeout(timeout);
                    setTimeout(() => {
                        transport.close();
                        resolve();
                    }, 100);
                }
            } catch (e) {
                reject(e);
            }
        });

        transport.onError((err) => {
            clearTimeout(timeout);
            reject(new Error(`${testName}: ${err}`));
        });

        // Send CONNECT
        console.log(`  ${testName}: Sending CONNECT`);
        transport.send(createConnectPacket(version, clientId));
    });
}

/**
 * Run a publish/subscribe test
 */
async function runPubSubTest(transport, version, testName) {
    const clientId = `test-pubsub-${version}-${Date.now()}`;
    const topic = `test/topic/${Date.now()}`;
    const message = `Hello MQTT ${version}!`;
    let packetId = 1;

    return new Promise((resolve, reject) => {
        const timeout = setTimeout(() => {
            reject(new Error(`${testName}: Timeout`));
        }, TEST_TIMEOUT);

        let state = 'connecting';

        transport.onMessage((data) => {
            try {
                const packetType = data[0] >> 4;

                if (packetType === 2 && state === 'connecting') { // CONNACK
                    console.log(`  ${testName}: Received CONNACK`);
                    state = 'subscribing';

                    // Subscribe to topic
                    transport.send(createSubscribePacket(version, packetId++, topic));
                    console.log(`  ${testName}: Sent SUBSCRIBE to ${topic}`);
                }
                else if (packetType === 9 && state === 'subscribing') { // SUBACK
                    console.log(`  ${testName}: Received SUBACK`);
                    state = 'publishing';

                    // Publish message
                    transport.send(createPublishPacket(version, topic, message));
                    console.log(`  ${testName}: Sent PUBLISH to ${topic}`);
                }
                else if (packetType === 3 && state === 'publishing') { // PUBLISH (received)
                    console.log(`  ${testName}: Received PUBLISH`);

                    // Extract and verify message
                    // Skip fixed header, get topic length
                    let offset = 2; // Skip fixed header (assuming remaining length fits in 1 byte)
                    const topicLen = (data[offset] << 8) | data[offset + 1];
                    offset += 2 + topicLen;

                    // For v5.0, skip properties
                    if (version === 'v5.0') {
                        const propsLen = data[offset];
                        offset += 1 + propsLen;
                    }

                    const receivedPayload = Buffer.from(data.slice(offset)).toString('utf8');
                    console.log(`  ${testName}: Received message: "${receivedPayload}"`);

                    if (receivedPayload === message) {
                        console.log(`  ${testName}: Message verified!`);
                    }

                    // Disconnect
                    transport.send(createDisconnectPacket(version));
                    clearTimeout(timeout);
                    setTimeout(() => {
                        transport.close();
                        resolve();
                    }, 100);
                }
            } catch (e) {
                reject(e);
            }
        });

        transport.onError((err) => {
            clearTimeout(timeout);
            reject(new Error(`${testName}: ${err}`));
        });

        // Send CONNECT
        console.log(`  ${testName}: Sending CONNECT`);
        transport.send(createConnectPacket(version, clientId));
    });
}

// ============================================================================
// Test Suites
// ============================================================================

async function runTests() {
    let broker;
    let passed = 0;
    let failed = 0;
    const connInfo = getConnectionInfo();

    console.log('Starting MQTT broker...');
    try {
        broker = await startBroker();
        console.log('Broker started\n');
    } catch (e) {
        console.error('Failed to start broker:', e.message);
        process.exit(1);
    }

    // Wait for broker to be fully ready
    await new Promise(r => setTimeout(r, 500));

    const tests = [
        // TCP tests
        {
            name: 'TCP v3.1.1 Connect',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await runConnectTest(transport, 'v3.1.1', 'TCP v3.1.1');
            }
        },
        {
            name: 'TCP v5.0 Connect',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await runConnectTest(transport, 'v5.0', 'TCP v5.0');
            }
        },
        {
            name: 'TCP v3.1.1 PubSub',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await runPubSubTest(transport, 'v3.1.1', 'TCP v3.1.1 PubSub');
            }
        },
        {
            name: 'TCP v5.0 PubSub',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await runPubSubTest(transport, 'v5.0', 'TCP v5.0 PubSub');
            }
        },

        // TLS tests
        {
            name: 'TLS v3.1.1 Connect',
            run: async () => {
                const transport = new NodeTlsTransport({
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                await transport.connect(connInfo.tls.host, connInfo.tls.port);
                await runConnectTest(transport, 'v3.1.1', 'TLS v3.1.1');
            }
        },
        {
            name: 'TLS v5.0 Connect',
            run: async () => {
                const transport = new NodeTlsTransport({
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                await transport.connect(connInfo.tls.host, connInfo.tls.port);
                await runConnectTest(transport, 'v5.0', 'TLS v5.0');
            }
        },
        {
            name: 'TLS v3.1.1 PubSub',
            run: async () => {
                const transport = new NodeTlsTransport({
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                await transport.connect(connInfo.tls.host, connInfo.tls.port);
                await runPubSubTest(transport, 'v3.1.1', 'TLS v3.1.1 PubSub');
            }
        },
        {
            name: 'TLS v5.0 PubSub',
            run: async () => {
                const transport = new NodeTlsTransport({
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                await transport.connect(connInfo.tls.host, connInfo.tls.port);
                await runPubSubTest(transport, 'v5.0', 'TLS v5.0 PubSub');
            }
        },

        // WebSocket tests
        {
            name: 'WS v3.1.1 Connect',
            run: async () => {
                const transport = new NodeWebSocketTransport();
                await transport.connect(connInfo.ws);
                await runConnectTest(transport, 'v3.1.1', 'WS v3.1.1');
            }
        },
        {
            name: 'WS v5.0 Connect',
            run: async () => {
                const transport = new NodeWebSocketTransport();
                await transport.connect(connInfo.ws);
                await runConnectTest(transport, 'v5.0', 'WS v5.0');
            }
        },
        {
            name: 'WS v3.1.1 PubSub',
            run: async () => {
                const transport = new NodeWebSocketTransport();
                await transport.connect(connInfo.ws);
                await runPubSubTest(transport, 'v3.1.1', 'WS v3.1.1 PubSub');
            }
        },
        {
            name: 'WS v5.0 PubSub',
            run: async () => {
                const transport = new NodeWebSocketTransport();
                await transport.connect(connInfo.ws);
                await runPubSubTest(transport, 'v5.0', 'WS v5.0 PubSub');
            }
        },

        // WSS tests
        {
            name: 'WSS v3.1.1 Connect',
            run: async () => {
                const WebSocket = require('ws');
                const transport = new NodeWebSocketTransport();
                // Create custom WebSocket with TLS options
                const ws = new WebSocket(connInfo.wss, ['mqtt'], {
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                ws.binaryType = 'arraybuffer';

                await new Promise((resolve, reject) => {
                    ws.on('open', resolve);
                    ws.on('error', reject);
                });

                // Manually wire up the transport
                transport.ws = ws;
                ws.on('message', (data) => {
                    if (transport.eventCallbacks.message) {
                        transport.eventCallbacks.message(new Uint8Array(data));
                    }
                });
                ws.on('close', () => {
                    if (transport.eventCallbacks.closed) {
                        transport.eventCallbacks.closed();
                    }
                });
                ws.on('error', (err) => {
                    if (transport.eventCallbacks.error) {
                        transport.eventCallbacks.error(err.message);
                    }
                });

                await runConnectTest(transport, 'v3.1.1', 'WSS v3.1.1');
            }
        },
        {
            name: 'WSS v5.0 Connect',
            run: async () => {
                const WebSocket = require('ws');
                const transport = new NodeWebSocketTransport();
                const ws = new WebSocket(connInfo.wss, ['mqtt'], {
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                ws.binaryType = 'arraybuffer';

                await new Promise((resolve, reject) => {
                    ws.on('open', resolve);
                    ws.on('error', reject);
                });

                transport.ws = ws;
                ws.on('message', (data) => {
                    if (transport.eventCallbacks.message) {
                        transport.eventCallbacks.message(new Uint8Array(data));
                    }
                });
                ws.on('close', () => {
                    if (transport.eventCallbacks.closed) {
                        transport.eventCallbacks.closed();
                    }
                });
                ws.on('error', (err) => {
                    if (transport.eventCallbacks.error) {
                        transport.eventCallbacks.error(err.message);
                    }
                });

                await runConnectTest(transport, 'v5.0', 'WSS v5.0');
            }
        },
        {
            name: 'WSS v3.1.1 PubSub',
            run: async () => {
                const WebSocket = require('ws');
                const transport = new NodeWebSocketTransport();
                const ws = new WebSocket(connInfo.wss, ['mqtt'], {
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                ws.binaryType = 'arraybuffer';

                await new Promise((resolve, reject) => {
                    ws.on('open', resolve);
                    ws.on('error', reject);
                });

                transport.ws = ws;
                ws.on('message', (data) => {
                    if (transport.eventCallbacks.message) {
                        transport.eventCallbacks.message(new Uint8Array(data));
                    }
                });
                ws.on('close', () => {
                    if (transport.eventCallbacks.closed) {
                        transport.eventCallbacks.closed();
                    }
                });
                ws.on('error', (err) => {
                    if (transport.eventCallbacks.error) {
                        transport.eventCallbacks.error(err.message);
                    }
                });

                await runPubSubTest(transport, 'v3.1.1', 'WSS v3.1.1 PubSub');
            }
        },
        {
            name: 'WSS v5.0 PubSub',
            run: async () => {
                const WebSocket = require('ws');
                const transport = new NodeWebSocketTransport();
                const ws = new WebSocket(connInfo.wss, ['mqtt'], {
                    ca: fs.readFileSync(connInfo.certs.ca),
                    rejectUnauthorized: true
                });
                ws.binaryType = 'arraybuffer';

                await new Promise((resolve, reject) => {
                    ws.on('open', resolve);
                    ws.on('error', reject);
                });

                transport.ws = ws;
                ws.on('message', (data) => {
                    if (transport.eventCallbacks.message) {
                        transport.eventCallbacks.message(new Uint8Array(data));
                    }
                });
                ws.on('close', () => {
                    if (transport.eventCallbacks.closed) {
                        transport.eventCallbacks.closed();
                    }
                });
                ws.on('error', (err) => {
                    if (transport.eventCallbacks.error) {
                        transport.eventCallbacks.error(err.message);
                    }
                });

                await runPubSubTest(transport, 'v5.0', 'WSS v5.0 PubSub');
            }
        }
    ];

    console.log(`Running ${tests.length} tests...\n`);

    for (const test of tests) {
        process.stdout.write(`TEST: ${test.name}... `);
        try {
            await test.run();
            console.log('PASSED');
            passed++;
        } catch (e) {
            console.log(`FAILED: ${e.message}`);
            failed++;
        }

        // Small delay between tests
        await new Promise(r => setTimeout(r, 100));
    }

    console.log(`\n${'='.repeat(50)}`);
    console.log(`Results: ${passed} passed, ${failed} failed`);
    console.log('='.repeat(50));

    // Cleanup
    console.log('\nStopping broker...');
    await stopBroker(broker);
    console.log('Done');

    process.exit(failed > 0 ? 1 : 0);
}

// Run tests
runTests().catch(e => {
    console.error('Test runner error:', e);
    process.exit(1);
});
