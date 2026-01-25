/**
 * MQTT Client for Node.js
 *
 * This module provides a full MQTT client implementation for Node.js
 * using WASM for packet encoding/decoding and native Node.js transports.
 *
 * Supports:
 * - WebSocket (ws/wss) via 'ws' library
 * - TCP via Node.js 'net' module
 * - TLS via Node.js 'tls' module
 */

/**
 * Parse URL to determine transport type and connection details
 */
function parseUrl(url) {
    const parsed = new URL(url);
    const protocol = parsed.protocol.replace(':', '');

    let transportType;
    let host = parsed.hostname;
    let port = parseInt(parsed.port) || null;

    switch (protocol) {
        case 'ws':
            transportType = 'websocket';
            port = port || 80;
            break;
        case 'wss':
            transportType = 'websocket';
            port = port || 443;
            break;
        case 'mqtt':
            transportType = 'tcp';
            port = port || 1883;
            break;
        case 'mqtts':
            transportType = 'tls';
            port = port || 8883;
            break;
        case 'tcp':
            transportType = 'tcp';
            port = port || 1883;
            break;
        case 'tls':
        case 'ssl':
            transportType = 'tls';
            port = port || 8883;
            break;
        default:
            throw new Error(`Unsupported protocol: ${protocol}`);
    }

    return { transportType, host, port, url };
}

/**
 * Node.js MQTT Client
 *
 * @example
 * const { NodeMqttClient } = require('mqtt-client-wasm/nodejs');
 *
 * const client = new NodeMqttClient({
 *     version: '5.0',
 *     autoPubResponse: true,
 *     autoPingResponse: true
 * });
 *
 * await client.connect('wss://broker.example.com:8884');
 * // or
 * await client.connect('mqtt://broker.example.com:1883');
 * // or
 * await client.connect('mqtts://broker.example.com:8883');
 */
class NodeMqttClient {
    constructor(config = {}) {
        this.config = {
            version: config.version || '5.0',
            autoPubResponse: config.autoPubResponse !== false,
            autoPingResponse: config.autoPingResponse !== false,
            keepAlive: config.keepAlive || 60,
            tlsOptions: config.tlsOptions || {}
        };

        this.transport = null;
        this.transportType = null;
        this.connected = false;
        this.buffer = Buffer.alloc(0);

        // Packet handling
        this.pendingRecv = [];
        this.receivedPackets = [];
        this.packetIdCounter = 1;
        this.usedPacketIds = new Set();

        // WASM module (lazy loaded)
        this.wasmModule = null;
        this.wasmConfig = null;

        // Event handlers
        this.eventHandlers = {
            connected: [],
            disconnected: [],
            message: [],
            error: []
        };

        // Timer management
        this.pingTimer = null;
        this.pingTimeoutTimer = null;
    }

    /**
     * Initialize WASM module
     */
    async initWasm() {
        if (this.wasmModule) return;

        const wasm = require('../pkg-nodejs/mqtt_client_wasm.js');
        wasm.init();

        this.wasmModule = wasm;
        this.wasmConfig = new wasm.WasmMqttConfig({
            version: this.config.version,
            autoPubResponse: this.config.autoPubResponse,
            autoPingResponse: this.config.autoPingResponse
        });

        // Create a temporary client for packet creation
        // We only use it for creating packets, not for transport
        this._packetHelper = new wasm.WasmMqttClient(this.wasmConfig);
    }

    /**
     * Connect to MQTT broker
     *
     * @param {string} url - Broker URL (ws://, wss://, mqtt://, mqtts://, tcp://, tls://)
     * @param {object} options - Connection options
     */
    async connect(url, options = {}) {
        await this.initWasm();

        const { transportType, host, port, url: fullUrl } = parseUrl(url);
        this.transportType = transportType;

        // Create transport
        await this._createTransport(transportType, host, port, fullUrl);

        // Send CONNECT packet
        const connectOptions = {
            clientId: options.clientId || `node-mqtt-${Date.now()}`,
            cleanStart: options.cleanStart !== false,
            keepAlive: options.keepAlive || this.config.keepAlive,
            userName: options.userName,
            password: options.password
        };

        // Add v5.0 properties if applicable
        if (this.config.version === '5.0') {
            if (options.sessionExpiryInterval !== undefined) {
                connectOptions.sessionExpiryInterval = options.sessionExpiryInterval;
            }
            if (options.receiveMaximum !== undefined) {
                connectOptions.receiveMaximum = options.receiveMaximum;
            }
        }

        const connectPacket = this._packetHelper.newConnectPacket(connectOptions);
        await this._sendPacket(connectPacket);

        // Wait for CONNACK
        const connackPacket = await this._recvPacket();
        const connack = this._packetHelper.asConnack(connackPacket);

        const isV5 = this.config.version === '5.0';
        const code = isV5 ? connack.reasonCode : connack.returnCode;

        if (code !== 0) {
            throw new Error(`Connection refused: code=${code}`);
        }

        this.connected = true;

        // Start ping timer based on keepAlive
        this._startPingTimer(connectOptions.keepAlive);

        // Emit connected event
        this._emit('connected', { sessionPresent: connack.sessionPresent });

        return {
            sessionPresent: connack.sessionPresent,
            assignedClientId: isV5 ? connack.assignedClientIdentifier : null
        };
    }

    /**
     * Create transport based on type
     */
    async _createTransport(transportType, host, port, url) {
        switch (transportType) {
            case 'websocket':
                await this._createWebSocketTransport(url);
                break;
            case 'tcp':
                await this._createTcpTransport(host, port);
                break;
            case 'tls':
                await this._createTlsTransport(host, port);
                break;
            default:
                throw new Error(`Unknown transport type: ${transportType}`);
        }
    }

    async _createWebSocketTransport(url) {
        const WebSocket = require('ws');

        return new Promise((resolve, reject) => {
            this.transport = new WebSocket(url, ['mqtt']);
            this.transport.binaryType = 'arraybuffer';

            this.transport.on('open', () => resolve());
            this.transport.on('error', (err) => {
                this._emit('error', err);
                reject(err);
            });
            this.transport.on('message', (data) => this._onData(Buffer.from(data)));
            this.transport.on('close', () => this._onClose());
        });
    }

    async _createTcpTransport(host, port) {
        const net = require('net');

        return new Promise((resolve, reject) => {
            this.transport = net.createConnection({ host, port }, () => resolve());
            this.transport.on('error', (err) => {
                this._emit('error', err);
                reject(err);
            });
            this.transport.on('data', (data) => this._onData(data));
            this.transport.on('close', () => this._onClose());
        });
    }

    async _createTlsTransport(host, port) {
        const tls = require('tls');

        return new Promise((resolve, reject) => {
            const options = {
                host,
                port,
                ...this.config.tlsOptions
            };

            this.transport = tls.connect(options, () => resolve());
            this.transport.on('error', (err) => {
                this._emit('error', err);
                reject(err);
            });
            this.transport.on('data', (data) => this._onData(data));
            this.transport.on('close', () => this._onClose());
        });
    }

    /**
     * Handle incoming data
     */
    _onData(data) {
        // Append to buffer
        this.buffer = Buffer.concat([this.buffer, data]);

        // Try to parse packets
        this._processBuffer();
    }

    /**
     * Process buffer and extract packets
     */
    _processBuffer() {
        while (this.buffer.length > 0) {
            // Try to parse a packet from buffer
            try {
                const result = this._tryParsePacket(this.buffer);
                if (!result) break; // Need more data

                const { packet, bytesConsumed } = result;
                this.buffer = this.buffer.slice(bytesConsumed);

                // Handle the packet
                this._handlePacket(packet);
            } catch (err) {
                this._emit('error', err);
                break;
            }
        }
    }

    /**
     * Try to parse a packet from buffer
     * Returns null if not enough data
     */
    _tryParsePacket(buffer) {
        if (buffer.length < 2) return null;

        // Read remaining length (variable length encoding)
        let multiplier = 1;
        let remainingLength = 0;
        let index = 1;

        while (index < buffer.length) {
            const byte = buffer[index];
            remainingLength += (byte & 0x7F) * multiplier;
            multiplier *= 128;
            index++;

            if ((byte & 0x80) === 0) break;
            if (multiplier > 128 * 128 * 128) {
                throw new Error('Malformed remaining length');
            }
        }

        const totalLength = index + remainingLength;
        if (buffer.length < totalLength) return null;

        // Extract packet bytes
        const packetBytes = buffer.slice(0, totalLength);

        // Use WASM to parse the packet
        const packet = this.wasmModule.WasmMqttPacket.fromBytes(
            new Uint8Array(packetBytes),
            this.config.version
        );

        return { packet, bytesConsumed: totalLength };
    }

    /**
     * Handle a received packet
     */
    _handlePacket(packet) {
        const packetType = packet.packetType();
        const { WasmPacketType } = this.wasmModule;

        // Reset ping timer on any packet received
        this._resetPingTimer();

        // Check if there's a pending recv
        if (this.pendingRecv.length > 0) {
            const resolver = this.pendingRecv.shift();
            resolver(packet);
            return;
        }

        // Handle specific packet types
        switch (packetType) {
            case WasmPacketType.Publish:
                this._handlePublish(packet);
                break;
            case WasmPacketType.Pingresp:
                this._handlePingresp();
                break;
            case WasmPacketType.Disconnect:
                this._handleDisconnect(packet);
                break;
            default:
                // Store for later recv()
                this.receivedPackets.push(packet);
        }
    }

    _handlePublish(packet) {
        const publish = this._packetHelper.asPublish(packet);
        const message = {
            topic: publish.topicName,
            payload: publish.payloadBytes(),
            qos: publish.qos,
            retain: publish.retain,
            packetId: publish.packetId
        };

        // Auto-respond to QoS > 0 if configured
        if (this.config.autoPubResponse && publish.qos > 0) {
            this._sendPubResponse(publish.qos, publish.packetId);
        }

        this._emit('message', message);

        // Also store for recv()
        this.receivedPackets.push(packet);
    }

    async _sendPubResponse(qos, packetId) {
        if (qos === 1) {
            const puback = this._packetHelper.newPubackPacket({ packetId });
            await this._sendPacket(puback);
        } else if (qos === 2) {
            const pubrec = this._packetHelper.newPubrecPacket({ packetId });
            await this._sendPacket(pubrec);
        }
    }

    _handlePingresp() {
        // Clear ping timeout timer
        if (this.pingTimeoutTimer) {
            clearTimeout(this.pingTimeoutTimer);
            this.pingTimeoutTimer = null;
        }
    }

    _handleDisconnect(packet) {
        this.connected = false;
        this._stopPingTimer();
        this._emit('disconnected', {});
    }

    /**
     * Handle connection close
     */
    _onClose() {
        this.connected = false;
        this._stopPingTimer();
        this._emit('disconnected', {});
    }

    /**
     * Start ping timer
     */
    _startPingTimer(keepAlive) {
        if (keepAlive <= 0) return;

        const interval = (keepAlive * 1000) / 2; // Send ping at half the keepAlive interval
        this.pingTimer = setInterval(() => this._sendPing(), interval);
    }

    _resetPingTimer() {
        // Ping timer continues as-is; this is just for tracking
    }

    _stopPingTimer() {
        if (this.pingTimer) {
            clearInterval(this.pingTimer);
            this.pingTimer = null;
        }
        if (this.pingTimeoutTimer) {
            clearTimeout(this.pingTimeoutTimer);
            this.pingTimeoutTimer = null;
        }
    }

    async _sendPing() {
        if (!this.connected) return;

        try {
            const pingreq = this._packetHelper.newPingreqPacket();
            await this._sendPacket(pingreq);

            // Set timeout for PINGRESP
            this.pingTimeoutTimer = setTimeout(() => {
                this._emit('error', new Error('Ping timeout'));
                this.disconnect();
            }, 10000); // 10 second timeout
        } catch (err) {
            this._emit('error', err);
        }
    }

    /**
     * Send a packet
     */
    async _sendPacket(packet) {
        const bytes = packet.toBytes();
        return this._send(Buffer.from(bytes));
    }

    /**
     * Send raw data
     */
    _send(data) {
        return new Promise((resolve, reject) => {
            if (!this.transport) {
                reject(new Error('Not connected'));
                return;
            }

            if (this.transportType === 'websocket') {
                this.transport.send(data, (err) => {
                    if (err) reject(err);
                    else resolve();
                });
            } else {
                // TCP/TLS
                this.transport.write(data, (err) => {
                    if (err) reject(err);
                    else resolve();
                });
            }
        });
    }

    /**
     * Receive next packet
     */
    async _recvPacket(timeout = 30000) {
        // Check if we have a buffered packet
        if (this.receivedPackets.length > 0) {
            return this.receivedPackets.shift();
        }

        // Wait for next packet
        return new Promise((resolve, reject) => {
            const timer = setTimeout(() => {
                const idx = this.pendingRecv.indexOf(resolve);
                if (idx !== -1) this.pendingRecv.splice(idx, 1);
                reject(new Error('Receive timeout'));
            }, timeout);

            this.pendingRecv.push((packet) => {
                clearTimeout(timer);
                resolve(packet);
            });
        });
    }

    /**
     * Subscribe to topics
     */
    async subscribe(topics, options = {}) {
        if (!this.connected) throw new Error('Not connected');

        const packetId = this._acquirePacketId();
        const subscriptions = Array.isArray(topics)
            ? topics.map(t => typeof t === 'string' ? { topic: t, qos: 0 } : t)
            : [typeof topics === 'string' ? { topic: topics, qos: 0 } : topics];

        const subscribePacket = this._packetHelper.newSubscribePacket({
            packetId,
            subscriptions
        });

        await this._sendPacket(subscribePacket);

        // Wait for SUBACK
        const subackPacket = await this._recvPacket();
        const suback = this._packetHelper.asSuback(subackPacket);

        this._releasePacketId(packetId);

        return {
            packetId: suback.packetId,
            reasonCodes: suback.reasonCodes()
        };
    }

    /**
     * Unsubscribe from topics
     */
    async unsubscribe(topics) {
        if (!this.connected) throw new Error('Not connected');

        const packetId = this._acquirePacketId();
        const topicList = Array.isArray(topics) ? topics : [topics];

        const unsubscribePacket = this._packetHelper.newUnsubscribePacket({
            packetId,
            topics: topicList
        });

        await this._sendPacket(unsubscribePacket);

        // Wait for UNSUBACK
        const unsubackPacket = await this._recvPacket();
        this._releasePacketId(packetId);

        return { packetId };
    }

    /**
     * Publish a message
     */
    async publish(topic, payload, options = {}) {
        if (!this.connected) throw new Error('Not connected');

        const qos = options.qos || 0;
        const packetId = qos > 0 ? this._acquirePacketId() : undefined;

        const publishPacket = this._packetHelper.newPublishPacket({
            topicName: topic,
            payload: typeof payload === 'string' ? payload : Array.from(payload),
            qos,
            retain: options.retain || false,
            packetId
        });

        await this._sendPacket(publishPacket);

        // For QoS 1, wait for PUBACK
        if (qos === 1) {
            const pubackPacket = await this._recvPacket();
            this._releasePacketId(packetId);
        }
        // For QoS 2, handle full exchange
        else if (qos === 2) {
            const pubrecPacket = await this._recvPacket();
            const pubrel = this._packetHelper.newPubrelPacket({ packetId });
            await this._sendPacket(pubrel);
            const pubcompPacket = await this._recvPacket();
            this._releasePacketId(packetId);
        }
    }

    /**
     * Disconnect from broker
     */
    async disconnect(options = {}) {
        if (!this.connected) return;

        try {
            const disconnectPacket = this._packetHelper.newDisconnectPacket({
                reasonCode: options.reasonCode || 0
            });
            await this._sendPacket(disconnectPacket);
        } catch (err) {
            // Ignore errors during disconnect
        }

        this._stopPingTimer();
        this.connected = false;

        if (this.transport) {
            if (this.transportType === 'websocket') {
                this.transport.close();
            } else {
                this.transport.end();
            }
            this.transport = null;
        }
    }

    /**
     * Packet ID management
     */
    _acquirePacketId() {
        for (let i = 0; i < 65535; i++) {
            const id = this.packetIdCounter;
            this.packetIdCounter = (this.packetIdCounter % 65535) + 1;

            if (!this.usedPacketIds.has(id)) {
                this.usedPacketIds.add(id);
                return id;
            }
        }
        throw new Error('No available packet IDs');
    }

    _releasePacketId(packetId) {
        this.usedPacketIds.delete(packetId);
    }

    /**
     * Event handling
     */
    on(event, handler) {
        if (this.eventHandlers[event]) {
            this.eventHandlers[event].push(handler);
        }
        return this;
    }

    off(event, handler) {
        if (this.eventHandlers[event]) {
            const idx = this.eventHandlers[event].indexOf(handler);
            if (idx !== -1) {
                this.eventHandlers[event].splice(idx, 1);
            }
        }
        return this;
    }

    _emit(event, data) {
        if (this.eventHandlers[event]) {
            for (const handler of this.eventHandlers[event]) {
                try {
                    handler(data);
                } catch (err) {
                    console.error(`Error in ${event} handler:`, err);
                }
            }
        }
    }
}

module.exports = { NodeMqttClient, parseUrl };
