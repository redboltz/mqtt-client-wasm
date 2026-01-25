/**
 * Timer tests for Node.js MQTT client
 *
 * Tests keep-alive behavior at the MQTT protocol level.
 * Note: These tests verify protocol-level timer behavior, not WASM internal timers.
 * The WASM timer code is tested via native Rust tests.
 */

const fs = require('fs');
const { startBroker, stopBroker, getConnectionInfo } = require('./broker-helper');
const {
    init,
    NodeTcpTransport
} = require('../index.js');

// Initialize WASM
init();

/**
 * Create MQTT v3.1.1 CONNECT packet with specified keep-alive
 */
function createConnectPacket(clientId, keepAlive) {
    const protocolName = Buffer.from('MQTT');
    const protocolLevel = 4; // v3.1.1
    const connectFlags = 0x02; // Clean session
    const clientIdBuf = Buffer.from(clientId, 'utf8');

    const variableHeader = Buffer.alloc(10);
    variableHeader.writeUInt16BE(4, 0);
    protocolName.copy(variableHeader, 2);
    variableHeader.writeUInt8(protocolLevel, 6);
    variableHeader.writeUInt8(connectFlags, 7);
    variableHeader.writeUInt16BE(keepAlive, 8);

    const payload = Buffer.alloc(2 + clientIdBuf.length);
    payload.writeUInt16BE(clientIdBuf.length, 0);
    clientIdBuf.copy(payload, 2);

    const remainingLength = variableHeader.length + payload.length;
    const fixedHeader = Buffer.from([0x10, remainingLength]);

    return Buffer.concat([fixedHeader, variableHeader, payload]);
}

/**
 * Create PINGREQ packet
 */
function createPingreqPacket() {
    return Buffer.from([0xC0, 0x00]);
}

/**
 * Create DISCONNECT packet
 */
function createDisconnectPacket() {
    return Buffer.from([0xE0, 0x00]);
}

/**
 * Test: Broker responds to PINGREQ with PINGRESP
 */
async function testPingResponse(transport, connInfo) {
    return new Promise((resolve, reject) => {
        const clientId = `ping-test-${Date.now()}`;
        const timeout = setTimeout(() => {
            reject(new Error('Timeout waiting for PINGRESP'));
        }, 5000);

        let state = 'connecting';

        transport.onMessage((data) => {
            const packetType = data[0] >> 4;

            if (packetType === 2 && state === 'connecting') { // CONNACK
                console.log('  Received CONNACK, sending PINGREQ...');
                state = 'pinging';
                transport.send(createPingreqPacket());
            }
            else if (packetType === 13 && state === 'pinging') { // PINGRESP
                console.log('  Received PINGRESP - ping/pong working!');
                clearTimeout(timeout);
                transport.send(createDisconnectPacket());
                setTimeout(() => {
                    transport.close();
                    resolve();
                }, 100);
            }
        });

        transport.onError((err) => {
            clearTimeout(timeout);
            reject(new Error(`Transport error: ${err}`));
        });

        console.log('  Sending CONNECT');
        transport.send(createConnectPacket(clientId, 60));
    });
}

/**
 * Test: Multiple PINGREQ/PINGRESP cycles
 */
async function testMultiplePings(transport, connInfo) {
    return new Promise((resolve, reject) => {
        const clientId = `multi-ping-${Date.now()}`;
        const timeout = setTimeout(() => {
            reject(new Error('Timeout in multi-ping test'));
        }, 10000);

        let state = 'connecting';
        let pingCount = 0;
        const targetPings = 3;

        transport.onMessage((data) => {
            const packetType = data[0] >> 4;

            if (packetType === 2 && state === 'connecting') { // CONNACK
                console.log('  Received CONNACK, starting ping cycles...');
                state = 'pinging';
                transport.send(createPingreqPacket());
            }
            else if (packetType === 13 && state === 'pinging') { // PINGRESP
                pingCount++;
                console.log(`  Received PINGRESP #${pingCount}`);

                if (pingCount >= targetPings) {
                    console.log('  Multiple ping cycles completed!');
                    clearTimeout(timeout);
                    transport.send(createDisconnectPacket());
                    setTimeout(() => {
                        transport.close();
                        resolve();
                    }, 100);
                } else {
                    // Send another PINGREQ
                    setTimeout(() => {
                        transport.send(createPingreqPacket());
                    }, 100);
                }
            }
        });

        transport.onError((err) => {
            clearTimeout(timeout);
            reject(new Error(`Transport error: ${err}`));
        });

        console.log('  Sending CONNECT');
        transport.send(createConnectPacket(clientId, 60));
    });
}

/**
 * Test: Graceful disconnect (timer should not cause issues)
 */
async function testGracefulDisconnect(transport, connInfo) {
    return new Promise((resolve, reject) => {
        const clientId = `graceful-${Date.now()}`;
        const timeout = setTimeout(() => {
            reject(new Error('Timeout in graceful disconnect test'));
        }, 5000);

        let state = 'connecting';

        transport.onMessage((data) => {
            const packetType = data[0] >> 4;

            if (packetType === 2 && state === 'connecting') { // CONNACK
                console.log('  Received CONNACK, disconnecting gracefully...');
                state = 'disconnecting';
                transport.send(createDisconnectPacket());
                clearTimeout(timeout);
                setTimeout(() => {
                    console.log('  Graceful disconnect successful');
                    transport.close();
                    resolve();
                }, 200);
            }
        });

        transport.onError((err) => {
            clearTimeout(timeout);
            reject(new Error(`Transport error: ${err}`));
        });

        console.log('  Sending CONNECT with short keep-alive');
        transport.send(createConnectPacket(clientId, 5)); // 5 second keep-alive
    });
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
            name: 'PINGREQ/PINGRESP cycle',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await testPingResponse(transport, connInfo);
            }
        },
        {
            name: 'Multiple PINGREQ/PINGRESP cycles',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await testMultiplePings(transport, connInfo);
            }
        },
        {
            name: 'Graceful disconnect with keep-alive',
            run: async () => {
                const transport = new NodeTcpTransport();
                await transport.connect(connInfo.tcp.host, connInfo.tcp.port);
                await testGracefulDisconnect(transport, connInfo);
            }
        }
    ];

    console.log(`Running ${tests.length} timer tests...\n`);

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
    console.log(`Timer Tests: ${passed} passed, ${failed} failed`);
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
