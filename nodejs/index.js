/**
 * MQTT Client for Node.js
 *
 * This module provides MQTT client functionality for Node.js environments
 * using the same WASM core as the browser version.
 *
 * Supports:
 * - WebSocket (ws/wss) via 'ws' library
 * - TCP via Node.js 'net' module
 * - TLS via Node.js 'tls' module
 *
 * All transports integrate with the WASM client's state machine, timers,
 * and automatic packet handling (PINGREQ, PUBACK, etc.).
 */

const {
    WasmMqttClient,
    WasmMqttConfig,
    WasmMqttPacket,
    WasmPacketType,
    JsTransport,
    createClientWithJsTransport,
    init
} = require('../pkg-nodejs/mqtt_client_wasm.js');

// Transport types
const TransportType = {
    WEBSOCKET: 'websocket',
    TCP: 'tcp',
    TLS: 'tls'
};

/**
 * Base class for Node.js transports
 * Provides integration with WASM client via JsTransport bridge
 *
 * Usage:
 * 1. Create transport instance
 * 2. Call createClientWithTransport(config, transport) to create client
 * 3. Call transport.connect() to establish connection
 * 4. Use client.send() and client.recv() for MQTT operations
 */
class BaseTransport {
    constructor() {
        // JsTransport is set by createClientWithTransport
        this.jsTransport = null;
    }

    /**
     * Notify WASM that connection is established
     * Called automatically by transport implementations when connected
     */
    notifyConnected() {
        if (this.jsTransport) {
            this.jsTransport.notifyConnected();
        }
    }

    /**
     * Notify WASM that data was received
     * Called automatically by transport implementations
     * @param {Uint8Array} data - The received data
     */
    notifyMessage(data) {
        if (this.jsTransport) {
            this.jsTransport.notifyMessage(data);
        }
    }

    /**
     * Notify WASM that an error occurred
     * Called automatically by transport implementations
     * @param {string} error - The error message
     */
    notifyError(error) {
        if (this.jsTransport) {
            this.jsTransport.notifyError(error);
        }
    }

    /**
     * Notify WASM that connection is closed
     * Called automatically by transport implementations
     */
    notifyClosed() {
        if (this.jsTransport) {
            this.jsTransport.notifyClosed();
        }
    }

    // Abstract methods to be implemented by subclasses
    _doSend(data) {
        throw new Error('_doSend must be implemented by subclass');
    }

    _doClose() {
        throw new Error('_doClose must be implemented by subclass');
    }
}

/**
 * WebSocket transport using 'ws' library
 */
class NodeWebSocketTransport extends BaseTransport {
    constructor(options = {}) {
        super();
        this.ws = null;
        this.options = options; // TLS options like ca, cert, key, rejectUnauthorized
        // Legacy callback support for direct usage
        this.eventCallbacks = {
            connected: null,
            message: null,
            error: null,
            closed: null
        };
    }

    async connect(url) {
        const WebSocket = require('ws');

        return new Promise((resolve, reject) => {
            try {
                // Pass TLS options for secure WebSocket connections
                this.ws = new WebSocket(url, ['mqtt'], this.options);
                this.ws.binaryType = 'arraybuffer';

                this.ws.on('open', () => {
                    // Notify WASM transport
                    this.notifyConnected();
                    // Legacy callback
                    if (this.eventCallbacks.connected) {
                        this.eventCallbacks.connected();
                    }
                    resolve();
                });

                this.ws.on('message', (data) => {
                    const uint8Array = new Uint8Array(data);
                    // Notify WASM transport
                    this.notifyMessage(uint8Array);
                    // Legacy callback
                    if (this.eventCallbacks.message) {
                        this.eventCallbacks.message(uint8Array);
                    }
                });

                this.ws.on('error', (err) => {
                    const errMsg = err.message || 'WebSocket error';
                    // Notify WASM transport
                    this.notifyError(errMsg);
                    // Legacy callback
                    if (this.eventCallbacks.error) {
                        this.eventCallbacks.error(errMsg);
                    }
                    reject(err);
                });

                this.ws.on('close', () => {
                    // Notify WASM transport
                    this.notifyClosed();
                    // Legacy callback
                    if (this.eventCallbacks.closed) {
                        this.eventCallbacks.closed();
                    }
                });

            } catch (err) {
                reject(err);
            }
        });
    }

    _doSend(data) {
        if (this.ws && this.ws.readyState === 1) { // OPEN
            this.ws.send(data);
        }
    }

    _doClose() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    // Legacy API support
    send(data) {
        this._doSend(data);
    }

    close() {
        this._doClose();
    }

    onConnected(callback) {
        this.eventCallbacks.connected = callback;
    }

    onMessage(callback) {
        this.eventCallbacks.message = callback;
    }

    onError(callback) {
        this.eventCallbacks.error = callback;
    }

    onClosed(callback) {
        this.eventCallbacks.closed = callback;
    }
}

/**
 * TCP transport using Node.js 'net' module
 */
class NodeTcpTransport extends BaseTransport {
    constructor() {
        super();
        this.socket = null;
        // Legacy callback support for direct usage
        this.eventCallbacks = {
            connected: null,
            message: null,
            error: null,
            closed: null
        };
    }

    async connect(host, port) {
        const net = require('net');

        return new Promise((resolve, reject) => {
            this.socket = net.createConnection({ host, port }, () => {
                // Notify WASM transport
                this.notifyConnected();
                // Legacy callback
                if (this.eventCallbacks.connected) {
                    this.eventCallbacks.connected();
                }
                resolve();
            });

            this.socket.on('data', (data) => {
                const uint8Array = new Uint8Array(data);
                // Notify WASM transport
                this.notifyMessage(uint8Array);
                // Legacy callback
                if (this.eventCallbacks.message) {
                    this.eventCallbacks.message(uint8Array);
                }
            });

            this.socket.on('error', (err) => {
                const errMsg = err.message || 'TCP error';
                // Notify WASM transport
                this.notifyError(errMsg);
                // Legacy callback
                if (this.eventCallbacks.error) {
                    this.eventCallbacks.error(errMsg);
                }
                reject(err);
            });

            this.socket.on('close', () => {
                // Notify WASM transport
                this.notifyClosed();
                // Legacy callback
                if (this.eventCallbacks.closed) {
                    this.eventCallbacks.closed();
                }
            });
        });
    }

    _doSend(data) {
        if (this.socket && !this.socket.destroyed) {
            this.socket.write(Buffer.from(data));
        }
    }

    _doClose() {
        if (this.socket) {
            this.socket.end();
            this.socket = null;
        }
    }

    // Legacy API support
    send(data) {
        this._doSend(data);
    }

    close() {
        this._doClose();
    }

    onConnected(callback) {
        this.eventCallbacks.connected = callback;
    }

    onMessage(callback) {
        this.eventCallbacks.message = callback;
    }

    onError(callback) {
        this.eventCallbacks.error = callback;
    }

    onClosed(callback) {
        this.eventCallbacks.closed = callback;
    }
}

/**
 * TLS transport using Node.js 'tls' module
 */
class NodeTlsTransport extends BaseTransport {
    constructor(options = {}) {
        super();
        this.socket = null;
        this.options = options; // TLS options like ca, cert, key, rejectUnauthorized
        // Legacy callback support for direct usage
        this.eventCallbacks = {
            connected: null,
            message: null,
            error: null,
            closed: null
        };
    }

    async connect(host, port) {
        const tls = require('tls');

        return new Promise((resolve, reject) => {
            const tlsOptions = {
                host,
                port,
                ...this.options
            };

            this.socket = tls.connect(tlsOptions, () => {
                // Notify WASM transport
                this.notifyConnected();
                // Legacy callback
                if (this.eventCallbacks.connected) {
                    this.eventCallbacks.connected();
                }
                resolve();
            });

            this.socket.on('data', (data) => {
                const uint8Array = new Uint8Array(data);
                // Notify WASM transport
                this.notifyMessage(uint8Array);
                // Legacy callback
                if (this.eventCallbacks.message) {
                    this.eventCallbacks.message(uint8Array);
                }
            });

            this.socket.on('error', (err) => {
                const errMsg = err.message || 'TLS error';
                // Notify WASM transport
                this.notifyError(errMsg);
                // Legacy callback
                if (this.eventCallbacks.error) {
                    this.eventCallbacks.error(errMsg);
                }
                reject(err);
            });

            this.socket.on('close', () => {
                // Notify WASM transport
                this.notifyClosed();
                // Legacy callback
                if (this.eventCallbacks.closed) {
                    this.eventCallbacks.closed();
                }
            });
        });
    }

    _doSend(data) {
        if (this.socket && !this.socket.destroyed) {
            this.socket.write(Buffer.from(data));
        }
    }

    _doClose() {
        if (this.socket) {
            this.socket.end();
            this.socket = null;
        }
    }

    // Legacy API support
    send(data) {
        this._doSend(data);
    }

    close() {
        this._doClose();
    }

    onConnected(callback) {
        this.eventCallbacks.connected = callback;
    }

    onMessage(callback) {
        this.eventCallbacks.message = callback;
    }

    onError(callback) {
        this.eventCallbacks.error = callback;
    }

    onClosed(callback) {
        this.eventCallbacks.closed = callback;
    }
}

/**
 * Create an MQTT client with a custom transport
 *
 * This creates a WasmMqttClient that uses the provided transport for
 * communication, while leveraging the WASM client's state machine,
 * timers, and automatic packet handling.
 *
 * IMPORTANT: Call this BEFORE connecting the transport. The JsTransport
 * needs to be created first so it can receive connection notifications.
 *
 * @param {WasmMqttConfig} config - The MQTT client configuration
 * @param {BaseTransport} transport - The transport to use (must extend BaseTransport)
 * @returns {WasmMqttClient} The configured MQTT client
 *
 * @example
 * const transport = new NodeTcpTransport();
 * const config = new WasmMqttConfig({ version: '3.1.1' });
 * const client = createClientWithTransport(config, transport);
 *
 * // Connect transport AFTER creating the client
 * await transport.connect('localhost', 1883);
 *
 * // Now use standard WASM client API
 * const connectPacket = client.newConnectPacket({ clientId: 'test' });
 * await client.send(connectPacket);
 * const connack = await client.recv();
 */
function createClientWithTransport(config, transport) {
    // Create the JsTransport bridge - this must be done before connecting
    const jsTransport = new JsTransport();

    // Set up callbacks from WASM to JavaScript transport
    jsTransport.setCallbacks({
        onSend: (data) => {
            transport._doSend(new Uint8Array(data));
        },
        onClose: () => {
            transport._doClose();
        }
    });

    // Store reference so transport can notify events
    transport.jsTransport = jsTransport;

    // Create the client with the JsTransport
    return createClientWithJsTransport(config, jsTransport);
}

module.exports = {
    // WASM exports
    WasmMqttClient,
    WasmMqttConfig,
    WasmMqttPacket,
    WasmPacketType,
    JsTransport,
    createClientWithJsTransport,
    init,

    // Transport types
    TransportType,

    // Transport implementations
    NodeWebSocketTransport,
    NodeTcpTransport,
    NodeTlsTransport,

    // Helper function (recommended for most users)
    createClientWithTransport
};
