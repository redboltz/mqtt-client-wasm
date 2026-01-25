/**
 * Transport Integration Tests
 *
 * Tests the new transport abstraction that integrates JavaScript transports
 * (TCP, TLS, WS, WSS) with the WASM client's state machine and timers.
 *
 * This provides the same API for browser and Node.js:
 * - client.send(packet)
 * - client.recv()
 * - Automatic PINGREQ/PINGRESP handling
 * - Automatic QoS response handling
 */

const fs = require('fs');
const { startBroker, stopBroker, getConnectionInfo } = require('./broker-helper');
const {
    init,
    WasmMqttConfig,
    WasmPacketType,
    NodeTcpTransport,
    NodeTlsTransport,
    NodeWebSocketTransport,
    createClientWithTransport
} = require('../index.js');

// Initialize WASM
init();

/**
 * Test: TCP transport with WASM client state machine
 */
async function testTcpWithStateMachine(connInfo, version) {
    const transport = new NodeTcpTransport();

    const config = new WasmMqttConfig({ version });
    const client = createClientWithTransport(config, transport);

    // Connect the transport after creating the client
    // This ensures the JsTransport callbacks are set up first
    await transport.connect(connInfo.tcp.host, connInfo.tcp.port);

    // Wait a bit for the async processors to start
    await new Promise(r => setTimeout(r, 50));

    // Create and send CONNECT packet
    const connectPacket = client.newConnectPacket({
        clientId: `tcp-test-${Date.now()}`,
        cleanSession: true,
        keepAlive: 60
    });
    await client.send(connectPacket);

    // Receive CONNACK
    const connack = await client.recv();
    if (connack.packetType() !== WasmPacketType.Connack) {
        throw new Error(`Expected CONNACK, got ${connack.packetType()}`);
    }

    // Subscribe to a topic
    const packetId = await client.acquirePacketId();
    const subscribePacket = client.newSubscribePacket({
        packetId,
        subscriptions: [{ topic: 'test/topic', qos: 0 }]
    });
    await client.send(subscribePacket);

    // Receive SUBACK
    const suback = await client.recv();
    if (suback.packetType() !== WasmPacketType.Suback) {
        throw new Error(`Expected SUBACK, got ${suback.packetType()}`);
    }

    // Publish a message
    const publishPacket = client.newPublishPacket({
        topicName: 'test/topic',
        payload: 'Hello from transport test',
        qos: 0
    });
    await client.send(publishPacket);

    // Receive the published message (since we subscribed)
    const publish = await client.recv();
    if (publish.packetType() !== WasmPacketType.Publish) {
        throw new Error(`Expected PUBLISH, got ${publish.packetType()}`);
    }

    // Disconnect
    const disconnectPacket = client.newDisconnectPacket();
    await client.send(disconnectPacket);
    await client.close();
    transport.close();

    console.log(`  TCP ${version} pub/sub successful`);
}

/**
 * Test: TLS transport with WASM client state machine
 */
async function testTlsWithStateMachine(connInfo, version) {
    const caPath = connInfo.certs.ca;
    const ca = fs.readFileSync(caPath);

    const transport = new NodeTlsTransport({
        ca,
        rejectUnauthorized: false // Allow self-signed certs for testing
    });

    const config = new WasmMqttConfig({ version });
    const client = createClientWithTransport(config, transport);

    await transport.connect(connInfo.tls.host, connInfo.tls.port);
    await new Promise(r => setTimeout(r, 50));

    // Create and send CONNECT packet
    const connectPacket = client.newConnectPacket({
        clientId: `tls-test-${Date.now()}`,
        cleanSession: true,
        keepAlive: 60
    });
    await client.send(connectPacket);

    // Receive CONNACK
    const connack = await client.recv();
    if (connack.packetType() !== WasmPacketType.Connack) {
        throw new Error(`Expected CONNACK, got ${connack.packetType()}`);
    }

    // Disconnect
    const disconnectPacket = client.newDisconnectPacket();
    await client.send(disconnectPacket);
    await client.close();
    transport.close();

    console.log(`  TLS ${version} connect successful`);
}

/**
 * Test: WebSocket transport with WASM client state machine
 */
async function testWsWithStateMachine(connInfo, version) {
    const transport = new NodeWebSocketTransport();

    const config = new WasmMqttConfig({ version });
    const client = createClientWithTransport(config, transport);

    await transport.connect(connInfo.ws);
    await new Promise(r => setTimeout(r, 50));

    // Create and send CONNECT packet
    const connectPacket = client.newConnectPacket({
        clientId: `ws-test-${Date.now()}`,
        cleanSession: true,
        keepAlive: 60
    });
    await client.send(connectPacket);

    // Receive CONNACK
    const connack = await client.recv();
    if (connack.packetType() !== WasmPacketType.Connack) {
        throw new Error(`Expected CONNACK, got ${connack.packetType()}`);
    }

    // Disconnect
    const disconnectPacket = client.newDisconnectPacket();
    await client.send(disconnectPacket);
    await client.close();
    transport.close();

    console.log(`  WebSocket ${version} connect successful`);
}

/**
 * Test: WSS (secure WebSocket) transport with WASM client state machine
 */
async function testWssWithStateMachine(connInfo, version) {
    // Load CA certificate for proper TLS verification
    const ca = fs.readFileSync(connInfo.certs.ca);
    const transport = new NodeWebSocketTransport({ ca });

    const config = new WasmMqttConfig({ version });
    const client = createClientWithTransport(config, transport);

    await transport.connect(connInfo.wss);
    await new Promise(r => setTimeout(r, 50));

    // Create and send CONNECT packet
    const connectPacket = client.newConnectPacket({
        clientId: `wss-test-${Date.now()}`,
        cleanSession: true,
        keepAlive: 60
    });
    await client.send(connectPacket);

    // Receive CONNACK
    const connack = await client.recv();
    if (connack.packetType() !== WasmPacketType.Connack) {
        throw new Error(`Expected CONNACK, got ${connack.packetType()}`);
    }

    // Disconnect
    const disconnectPacket = client.newDisconnectPacket();
    await client.send(disconnectPacket);
    await client.close();
    transport.close();

    console.log(`  WSS ${version} connect successful`);
}

/**
 * Test: QoS 1 publish with automatic PUBACK
 */
async function testQos1WithAutoPuback(connInfo) {
    const transport = new NodeTcpTransport();

    // Enable auto pub response (default is true)
    const config = new WasmMqttConfig({ version: '3.1.1' });
    const client = createClientWithTransport(config, transport);

    await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
    await new Promise(r => setTimeout(r, 50));

    // Send CONNECT
    const connectPacket = client.newConnectPacket({
        clientId: `qos1-test-${Date.now()}`,
        cleanSession: true,
        keepAlive: 60
    });
    await client.send(connectPacket);

    // Receive CONNACK
    const connack = await client.recv();
    if (connack.packetType() !== WasmPacketType.Connack) {
        throw new Error(`Expected CONNACK, got ${connack.packetType()}`);
    }

    // Subscribe with QoS 1
    const subPacketId = await client.acquirePacketId();
    const subscribePacket = client.newSubscribePacket({
        packetId: subPacketId,
        subscriptions: [{ topic: 'test/qos1', qos: 1 }]
    });
    await client.send(subscribePacket);

    // Receive SUBACK
    const suback = await client.recv();
    if (suback.packetType() !== WasmPacketType.Suback) {
        throw new Error(`Expected SUBACK, got ${suback.packetType()}`);
    }

    // Publish QoS 1 message
    const pubPacketId = await client.acquirePacketId();
    const publishPacket = client.newPublishPacket({
        topicName: 'test/qos1',
        payload: 'QoS 1 message',
        qos: 1,
        packetId: pubPacketId
    });
    await client.send(publishPacket);

    // Receive the echoed PUBLISH first (broker echoes our message since we subscribed)
    // This comes before PUBACK because of broker ordering
    const echoPublish = await client.recv();
    if (echoPublish.packetType() !== WasmPacketType.Publish) {
        throw new Error(`Expected PUBLISH echo, got ${echoPublish.packetType()}`);
    }

    // The WASM client should have automatically sent PUBACK for the received QoS 1 message
    // We don't need to manually send it because auto_pub_response is enabled by default

    // Now receive PUBACK for our original publish
    const puback = await client.recv();
    if (puback.packetType() !== WasmPacketType.Puback) {
        throw new Error(`Expected PUBACK, got ${puback.packetType()}`);
    }

    // Disconnect
    const disconnectPacket = client.newDisconnectPacket();
    await client.send(disconnectPacket);
    await client.close();
    transport.close();

    console.log('  QoS 1 with auto PUBACK successful');
}

// ============================================================================
// Main
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

    await new Promise(r => setTimeout(r, 500));

    const tests = [
        {
            name: 'TCP v3.1.1 with state machine',
            run: () => testTcpWithStateMachine(connInfo, '3.1.1')
        },
        {
            name: 'TCP v5.0 with state machine',
            run: () => testTcpWithStateMachine(connInfo, '5.0')
        },
        {
            name: 'TLS v3.1.1 with state machine',
            run: () => testTlsWithStateMachine(connInfo, '3.1.1')
        },
        {
            name: 'TLS v5.0 with state machine',
            run: () => testTlsWithStateMachine(connInfo, '5.0')
        },
        {
            name: 'WebSocket v3.1.1 with state machine',
            run: () => testWsWithStateMachine(connInfo, '3.1.1')
        },
        {
            name: 'WebSocket v5.0 with state machine',
            run: () => testWsWithStateMachine(connInfo, '5.0')
        },
        {
            name: 'WSS v3.1.1 with state machine',
            run: () => testWssWithStateMachine(connInfo, '3.1.1')
        },
        {
            name: 'WSS v5.0 with state machine',
            run: () => testWssWithStateMachine(connInfo, '5.0')
        },
        {
            name: 'QoS 1 with auto PUBACK',
            run: () => testQos1WithAutoPuback(connInfo)
        }
    ];

    console.log(`Running ${tests.length} transport integration tests...\n`);

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
        await new Promise(r => setTimeout(r, 100));
    }

    console.log(`\n${'='.repeat(50)}`);
    console.log(`Transport Integration Tests: ${passed} passed, ${failed} failed`);
    console.log('='.repeat(50));

    console.log('\nStopping broker...');
    await stopBroker(broker);
    console.log('Done');

    process.exit(failed > 0 ? 1 : 0);
}

runTests().catch(e => {
    console.error('Test runner error:', e);
    process.exit(1);
});
