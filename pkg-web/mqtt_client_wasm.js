/* @ts-self-types="./mqtt_client_wasm.d.ts" */

/**
 * JavaScript Transport Bridge
 *
 * This struct bridges JavaScript transport implementations to the Rust UnderlyingLayerInterface.
 * It allows Node.js transports (TCP, TLS, WebSocket) to integrate with the WASM client's
 * state machine, timers, and automatic packet handling.
 *
 * The JsTransport stays in JavaScript and can be used to notify events.
 * A separate JsTransportHandle is created for use by the Rust client.
 */
export class JsTransport {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsTransportFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jstransport_free(ptr, 0);
    }
    /**
     * Create a new JavaScript transport bridge
     */
    constructor() {
        const ret = wasm.jstransport_new();
        this.__wbg_ptr = ret >>> 0;
        JsTransportFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Called by JavaScript when the transport is closed
     */
    notifyClosed() {
        wasm.jstransport_notifyClosed(this.__wbg_ptr);
    }
    /**
     * Called by JavaScript when the transport connects successfully
     */
    notifyConnected() {
        wasm.jstransport_notifyConnected(this.__wbg_ptr);
    }
    /**
     * Called by JavaScript when an error occurs
     * @param {string} error
     */
    notifyError(error) {
        const ptr0 = passStringToWasm0(error, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jstransport_notifyError(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Called by JavaScript when data is received from the transport
     * @param {Uint8Array} data
     */
    notifyMessage(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jstransport_notifyMessage(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Set the JavaScript callbacks for transport operations
     *
     * The callbacks object must implement:
     * - onSend(data: Uint8Array): void - called to send data via transport
     * - onClose(): void - called to close the transport
     * @param {JsTransportCallbacks} callbacks
     */
    setCallbacks(callbacks) {
        wasm.jstransport_setCallbacks(this.__wbg_ptr, callbacks);
    }
}
if (Symbol.dispose) JsTransport.prototype[Symbol.dispose] = JsTransport.prototype.free;

/**
 * WASM wrapper for V5.0 AUTH packet
 */
export class WasmAuthPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmAuthPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmAuthPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmAuthPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmauthpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get authenticationData() {
        const ret = wasm.wasmauthpacketv5_0_authenticationData(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {string | undefined}
     */
    get authenticationMethod() {
        const ret = wasm.wasmauthpacketv5_0_authenticationMethod(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmauthpacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmauthpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the AUTH packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmauthpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmAuthPacketV5_0.prototype[Symbol.dispose] = WasmAuthPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 CONNACK packet
 */
export class WasmConnackPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmConnackPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmConnackPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmConnackPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmconnackpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    isSuccess() {
        const ret = wasm.wasmconnackpacketv3_1_1_isSuccess(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get returnCode() {
        const ret = wasm.wasmconnackpacketv3_1_1_returnCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get sessionPresent() {
        const ret = wasm.wasmconnackpacketv3_1_1_sessionPresent(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) WasmConnackPacketV3_1_1.prototype[Symbol.dispose] = WasmConnackPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 CONNACK packet
 */
export class WasmConnackPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmConnackPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmConnackPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmConnackPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmconnackpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {string | undefined}
     */
    get assignedClientIdentifier() {
        const ret = wasm.wasmconnackpacketv5_0_assignedClientIdentifier(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get authenticationData() {
        const ret = wasm.wasmconnackpacketv5_0_authenticationData(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {string | undefined}
     */
    get authenticationMethod() {
        const ret = wasm.wasmconnackpacketv5_0_authenticationMethod(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {boolean}
     */
    isSuccess() {
        const ret = wasm.wasmconnackpacketv5_0_isSuccess(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number | undefined}
     */
    get maximumPacketSize() {
        const ret = wasm.wasmconnackpacketv5_0_maximumPacketSize(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    get maximumQos() {
        const ret = wasm.wasmconnackpacketv5_0_maximumQos(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmconnackpacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmconnackpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {number | undefined}
     */
    get receiveMaximum() {
        const ret = wasm.wasmconnackpacketv5_0_receiveMaximum(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {string | undefined}
     */
    get responseInformation() {
        const ret = wasm.wasmconnackpacketv5_0_responseInformation(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {boolean | undefined}
     */
    get retainAvailable() {
        const ret = wasm.wasmconnackpacketv5_0_retainAvailable(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @returns {number | undefined}
     */
    get serverKeepAlive() {
        const ret = wasm.wasmconnackpacketv5_0_serverKeepAlive(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {string | undefined}
     */
    get serverReference() {
        const ret = wasm.wasmconnackpacketv5_0_serverReference(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {number | undefined}
     */
    get sessionExpiryInterval() {
        const ret = wasm.wasmconnackpacketv5_0_sessionExpiryInterval(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {boolean}
     */
    get sessionPresent() {
        const ret = wasm.wasmconnackpacketv5_0_sessionPresent(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean | undefined}
     */
    get sharedSubscriptionAvailable() {
        const ret = wasm.wasmconnackpacketv5_0_sharedSubscriptionAvailable(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @returns {boolean | undefined}
     */
    get subscriptionIdentifiersAvailable() {
        const ret = wasm.wasmconnackpacketv5_0_subscriptionIdentifiersAvailable(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @returns {number | undefined}
     */
    get topicAliasMaximum() {
        const ret = wasm.wasmconnackpacketv5_0_topicAliasMaximum(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * Returns the user properties from the CONNACK packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmconnackpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean | undefined}
     */
    get wildcardSubscriptionAvailable() {
        const ret = wasm.wasmconnackpacketv5_0_wildcardSubscriptionAvailable(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
}
if (Symbol.dispose) WasmConnackPacketV5_0.prototype[Symbol.dispose] = WasmConnackPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 DISCONNECT packet
 */
export class WasmDisconnectPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmDisconnectPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmDisconnectPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDisconnectPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdisconnectpacketv3_1_1_free(ptr, 0);
    }
}
if (Symbol.dispose) WasmDisconnectPacketV3_1_1.prototype[Symbol.dispose] = WasmDisconnectPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 DISCONNECT packet
 */
export class WasmDisconnectPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmDisconnectPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmDisconnectPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDisconnectPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdisconnectpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmdisconnectpacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmdisconnectpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {string | undefined}
     */
    get serverReference() {
        const ret = wasm.wasmdisconnectpacketv5_0_serverReference(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {number | undefined}
     */
    get sessionExpiryInterval() {
        const ret = wasm.wasmdisconnectpacketv5_0_sessionExpiryInterval(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Returns the user properties from the DISCONNECT packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmdisconnectpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmDisconnectPacketV5_0.prototype[Symbol.dispose] = WasmDisconnectPacketV5_0.prototype.free;

/**
 * WASM-friendly wrapper around MqttClient
 */
export class WasmMqttClient {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmMqttClient.prototype);
        obj.__wbg_ptr = ptr;
        WasmMqttClientFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMqttClientFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmqttclient_free(ptr, 0);
    }
    /**
     * Acquire a packet ID
     * @returns {Promise<number | undefined>}
     */
    acquirePacketId() {
        const ret = wasm.wasmmqttclient_acquirePacketId(this.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to AUTH wrapper (V5.0 only)
     * Returns WasmAuthPacketV5_0 for V5.0 clients, null otherwise
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asAuth(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asAuth(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to CONNACK wrapper (version-aware)
     * Returns WasmConnackPacketV3_1_1 or WasmConnackPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asConnack(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asConnack(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to DISCONNECT wrapper (version-aware)
     * Returns WasmDisconnectPacketV3_1_1 or WasmDisconnectPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asDisconnect(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asDisconnect(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to PUBACK wrapper (version-aware)
     * Returns WasmPubackPacketV3_1_1 or WasmPubackPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asPuback(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asPuback(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to PUBCOMP wrapper (version-aware)
     * Returns WasmPubcompPacketV3_1_1 or WasmPubcompPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asPubcomp(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asPubcomp(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to PUBLISH wrapper (version-aware)
     * Returns WasmPublishPacketV3_1_1 or WasmPublishPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asPublish(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asPublish(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to PUBREC wrapper (version-aware)
     * Returns WasmPubrecPacketV3_1_1 or WasmPubrecPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asPubrec(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asPubrec(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to PUBREL wrapper (version-aware)
     * Returns WasmPubrelPacketV3_1_1 or WasmPubrelPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asPubrel(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asPubrel(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to SUBACK wrapper (version-aware)
     * Returns WasmSubackPacketV3_1_1 or WasmSubackPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asSuback(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asSuback(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Convert packet to UNSUBACK wrapper (version-aware)
     * Returns WasmUnsubackPacketV3_1_1 or WasmUnsubackPacketV5_0 based on client version
     * @param {WasmMqttPacket} packet
     * @returns {any}
     */
    asUnsuback(packet) {
        _assertClass(packet, WasmMqttPacket);
        const ret = wasm.wasmmqttclient_asUnsuback(this.__wbg_ptr, packet.__wbg_ptr);
        return ret;
    }
    /**
     * Close the connection
     * @returns {Promise<void>}
     */
    close() {
        const ret = wasm.wasmmqttclient_close(this.__wbg_ptr);
        return ret;
    }
    /**
     * Connect to MQTT broker
     * @param {string} url
     * @returns {Promise<void>}
     */
    connect(url) {
        const ptr0 = passStringToWasm0(url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmmqttclient_connect(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * Get connection state
     * @returns {Promise<boolean>}
     */
    isConnected() {
        const ret = wasm.wasmmqttclient_isConnected(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {WasmMqttConfig} config
     */
    constructor(config) {
        _assertClass(config, WasmMqttConfig);
        var ptr0 = config.__destroy_into_raw();
        const ret = wasm.wasmmqttclient_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        WasmMqttClientFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Create Auth packet (V5.0 only)
     * Returns error if called on V3.1.1 client
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newAuthPacket(options) {
        const ret = wasm.wasmmqttclient_newAuthPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Connect packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newConnectPacket(options) {
        const ret = wasm.wasmmqttclient_newConnectPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Disconnect packet (version-aware)
     * For V3.1.1: options are ignored (Disconnect has no fields)
     * For V5.0: reason_code, reason_string, session_expiry_interval, user_properties are used
     * Pass undefined/null for default options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newDisconnectPacket(options) {
        const ret = wasm.wasmmqttclient_newDisconnectPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Pingreq packet (version-aware)
     * @returns {WasmMqttPacket}
     */
    newPingreqPacket() {
        const ret = wasm.wasmmqttclient_newPingreqPacket(this.__wbg_ptr);
        return WasmMqttPacket.__wrap(ret);
    }
    /**
     * Create Puback packet (version-aware)
     * For V3.1.1: only packet_id is used
     * For V5.0: packet_id, reason_code, reason_string, user_properties are used
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newPubackPacket(options) {
        const ret = wasm.wasmmqttclient_newPubackPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Pubcomp packet (version-aware)
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newPubcompPacket(options) {
        const ret = wasm.wasmmqttclient_newPubcompPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Publish packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newPublishPacket(options) {
        const ret = wasm.wasmmqttclient_newPublishPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Pubrec packet (version-aware)
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newPubrecPacket(options) {
        const ret = wasm.wasmmqttclient_newPubrecPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Pubrel packet (version-aware)
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newPubrelPacket(options) {
        const ret = wasm.wasmmqttclient_newPubrelPacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Subscribe packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newSubscribePacket(options) {
        const ret = wasm.wasmmqttclient_newSubscribePacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create Unsubscribe packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    newUnsubscribePacket(options) {
        const ret = wasm.wasmmqttclient_newUnsubscribePacket(this.__wbg_ptr, options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Receive next packet
     * @returns {Promise<WasmMqttPacket>}
     */
    recv() {
        const ret = wasm.wasmmqttclient_recv(this.__wbg_ptr);
        return ret;
    }
    /**
     * Register a packet ID
     * @param {number} packet_id
     * @returns {Promise<boolean>}
     */
    registerPacketId(packet_id) {
        const ret = wasm.wasmmqttclient_registerPacketId(this.__wbg_ptr, packet_id);
        return ret;
    }
    /**
     * Release a packet ID
     * @param {number} packet_id
     * @returns {Promise<void>}
     */
    releasePacketId(packet_id) {
        const ret = wasm.wasmmqttclient_releasePacketId(this.__wbg_ptr, packet_id);
        return ret;
    }
    /**
     * Send MQTT packet
     * @param {WasmMqttPacket} packet
     * @returns {Promise<void>}
     */
    send(packet) {
        _assertClass(packet, WasmMqttPacket);
        var ptr0 = packet.__destroy_into_raw();
        const ret = wasm.wasmmqttclient_send(this.__wbg_ptr, ptr0);
        return ret;
    }
}
if (Symbol.dispose) WasmMqttClient.prototype[Symbol.dispose] = WasmMqttClient.prototype.free;

/**
 * WASM-friendly wrapper around MqttConfig
 */
export class WasmMqttConfig {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMqttConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmqttconfig_free(ptr, 0);
    }
    /**
     * Create a new MQTT configuration from a JSON object.
     *
     * # Example (JavaScript)
     * ```js
     * const config = new WasmMqttConfig({
     *     version: '5.0',
     *     autoPubResponse: true,
     *     autoPingResponse: true,
     *     pingreqSendIntervalMs: 30000,
     * });
     * ```
     * @param {any} options
     */
    constructor(options) {
        const ret = wasm.wasmmqttconfig_new(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        WasmMqttConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmMqttConfig.prototype[Symbol.dispose] = WasmMqttConfig.prototype.free;

/**
 * WASM wrapper for MQTT packets
 */
export class WasmMqttPacket {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmMqttPacket.prototype);
        obj.__wbg_ptr = ptr;
        WasmMqttPacketFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMqttPacketFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmqttpacket_free(ptr, 0);
    }
    /**
     * Parse packet from bytes
     * Note: This creates a temporary MQTT connection to use its parser
     * @param {Uint8Array} data
     * @param {string} version
     * @returns {WasmMqttPacket}
     */
    static fromBytes(data, version) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(version, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmmqttpacket_fromBytes(ptr0, len0, ptr1, len1);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Auth packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newAuthV50(options) {
        const ret = wasm.wasmmqttpacket_newAuthV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Connect packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newConnectV311(options) {
        const ret = wasm.wasmmqttpacket_newConnectV311(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Connect packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newConnectV50(options) {
        const ret = wasm.wasmmqttpacket_newConnectV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Disconnect packet
     * @returns {WasmMqttPacket}
     */
    static newDisconnectV311() {
        const ret = wasm.wasmmqttpacket_newDisconnectV311();
        return WasmMqttPacket.__wrap(ret);
    }
    /**
     * Create V5.0 Disconnect packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newDisconnectV50(options) {
        const ret = wasm.wasmmqttpacket_newDisconnectV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Pingreq packet
     * @returns {WasmMqttPacket}
     */
    static newPingreqV311() {
        const ret = wasm.wasmmqttpacket_newPingreqV311();
        return WasmMqttPacket.__wrap(ret);
    }
    /**
     * Create V5.0 Pingreq packet
     * @returns {WasmMqttPacket}
     */
    static newPingreqV50() {
        const ret = wasm.wasmmqttpacket_newPingreqV50();
        return WasmMqttPacket.__wrap(ret);
    }
    /**
     * Create V3.1.1 Puback packet
     * @param {number} packet_id
     * @returns {WasmMqttPacket}
     */
    static newPubackV311(packet_id) {
        const ret = wasm.wasmmqttpacket_newPubackV311(packet_id);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Puback packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPubackV50(options) {
        const ret = wasm.wasmmqttpacket_newPubackV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Pubcomp packet
     * @param {number} packet_id
     * @returns {WasmMqttPacket}
     */
    static newPubcompV311(packet_id) {
        const ret = wasm.wasmmqttpacket_newPubcompV311(packet_id);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Pubcomp packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPubcompV50(options) {
        const ret = wasm.wasmmqttpacket_newPubcompV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Publish packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPublishV311(options) {
        const ret = wasm.wasmmqttpacket_newPublishV311(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Publish packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPublishV50(options) {
        const ret = wasm.wasmmqttpacket_newPublishV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Pubrec packet
     * @param {number} packet_id
     * @returns {WasmMqttPacket}
     */
    static newPubrecV311(packet_id) {
        const ret = wasm.wasmmqttpacket_newPubrecV311(packet_id);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Pubrec packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPubrecV50(options) {
        const ret = wasm.wasmmqttpacket_newPubrecV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Pubrel packet
     * @param {number} packet_id
     * @returns {WasmMqttPacket}
     */
    static newPubrelV311(packet_id) {
        const ret = wasm.wasmmqttpacket_newPubrelV311(packet_id);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Pubrel packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newPubrelV50(options) {
        const ret = wasm.wasmmqttpacket_newPubrelV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Subscribe packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newSubscribeV311(options) {
        const ret = wasm.wasmmqttpacket_newSubscribeV311(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Subscribe packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newSubscribeV50(options) {
        const ret = wasm.wasmmqttpacket_newSubscribeV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V3.1.1 Unsubscribe packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newUnsubscribeV311(options) {
        const ret = wasm.wasmmqttpacket_newUnsubscribeV311(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Create V5.0 Unsubscribe packet from JSON options
     * @param {any} options
     * @returns {WasmMqttPacket}
     */
    static newUnsubscribeV50(options) {
        const ret = wasm.wasmmqttpacket_newUnsubscribeV50(options);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmMqttPacket.__wrap(ret[0]);
    }
    /**
     * Get packet type as enum
     * @returns {WasmPacketType}
     */
    packetType() {
        const ret = wasm.wasmmqttpacket_packetType(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get packet type as string (for debugging)
     * @returns {string}
     */
    packetTypeString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmmqttpacket_packetTypeString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Serialize packet to bytes
     * @returns {Uint8Array}
     */
    toBytes() {
        const ret = wasm.wasmmqttpacket_toBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) WasmMqttPacket.prototype[Symbol.dispose] = WasmMqttPacket.prototype.free;

/**
 * Packet type enum exposed to JavaScript
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14}
 */
export const WasmPacketType = Object.freeze({
    Connect: 0, "0": "Connect",
    Connack: 1, "1": "Connack",
    Publish: 2, "2": "Publish",
    Puback: 3, "3": "Puback",
    Pubrec: 4, "4": "Pubrec",
    Pubrel: 5, "5": "Pubrel",
    Pubcomp: 6, "6": "Pubcomp",
    Subscribe: 7, "7": "Subscribe",
    Suback: 8, "8": "Suback",
    Unsubscribe: 9, "9": "Unsubscribe",
    Unsuback: 10, "10": "Unsuback",
    Pingreq: 11, "11": "Pingreq",
    Pingresp: 12, "12": "Pingresp",
    Disconnect: 13, "13": "Disconnect",
    Auth: 14, "14": "Auth",
});

/**
 * WASM wrapper for V3.1.1 PUBACK packet
 */
export class WasmPubackPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubackPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubackPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubackPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubackpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubackpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubackPacketV3_1_1.prototype[Symbol.dispose] = WasmPubackPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 PUBACK packet
 */
export class WasmPubackPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubackPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubackPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubackPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubackpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubackpacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmpubackpacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmpubackpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the PUBACK packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmpubackpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubackPacketV5_0.prototype[Symbol.dispose] = WasmPubackPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 PUBCOMP packet
 */
export class WasmPubcompPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubcompPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubcompPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubcompPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubcomppacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubcomppacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubcompPacketV3_1_1.prototype[Symbol.dispose] = WasmPubcompPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 PUBCOMP packet
 */
export class WasmPubcompPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubcompPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubcompPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubcompPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubcomppacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubcomppacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmpubcomppacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmpubcomppacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the PUBCOMP packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmpubcomppacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubcompPacketV5_0.prototype[Symbol.dispose] = WasmPubcompPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 PUBLISH packet
 */
export class WasmPublishPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPublishPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmPublishPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPublishPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpublishpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get dup() {
        const ret = wasm.wasmpublishpacketv3_1_1_dup(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number | undefined}
     */
    get packetId() {
        const ret = wasm.wasmpublishpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {string | undefined}
     */
    get payload() {
        const ret = wasm.wasmpublishpacketv3_1_1_payload(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {Uint8Array}
     */
    payloadBytes() {
        const ret = wasm.wasmpublishpacketv3_1_1_payloadBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {number}
     */
    get qos() {
        const ret = wasm.wasmpublishpacketv3_1_1_qos(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get retain() {
        const ret = wasm.wasmpublishpacketv3_1_1_retain(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {string}
     */
    get topicName() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmpublishpacketv3_1_1_topicName(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WasmPublishPacketV3_1_1.prototype[Symbol.dispose] = WasmPublishPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 PUBLISH packet
 */
export class WasmPublishPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPublishPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmPublishPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPublishPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpublishpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {string | undefined}
     */
    get contentType() {
        const ret = wasm.wasmpublishpacketv5_0_contentType(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get correlationData() {
        const ret = wasm.wasmpublishpacketv5_0_correlationData(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {boolean}
     */
    get dup() {
        const ret = wasm.wasmpublishpacketv5_0_dup(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number | undefined}
     */
    get messageExpiryInterval() {
        const ret = wasm.wasmpublishpacketv5_0_messageExpiryInterval(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    get packetId() {
        const ret = wasm.wasmpublishpacketv5_0_packetId(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {string | undefined}
     */
    get payload() {
        const ret = wasm.wasmpublishpacketv5_0_payload(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {Uint8Array}
     */
    payloadBytes() {
        const ret = wasm.wasmpublishpacketv5_0_payloadBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {number | undefined}
     */
    get payloadFormatIndicator() {
        const ret = wasm.wasmpublishpacketv5_0_payloadFormatIndicator(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {number}
     */
    get qos() {
        const ret = wasm.wasmpublishpacketv5_0_qos(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get responseTopic() {
        const ret = wasm.wasmpublishpacketv5_0_responseTopic(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @returns {boolean}
     */
    get retain() {
        const ret = wasm.wasmpublishpacketv5_0_retain(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns the subscription identifiers from the PUBLISH packet.
     * These are set by the broker to indicate which subscriptions matched the message.
     * Multiple subscription identifiers can be present if the message matches multiple subscriptions.
     * @returns {Uint32Array}
     */
    subscriptionIdentifiers() {
        const ret = wasm.wasmpublishpacketv5_0_subscriptionIdentifiers(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {number | undefined}
     */
    get topicAlias() {
        const ret = wasm.wasmpublishpacketv5_0_topicAlias(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @returns {string}
     */
    get topicName() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmpublishpacketv5_0_topicName(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns true if the topic name was extracted from topic alias mapping.
     * When a PUBLISH packet is received with an empty topic name and a topic alias,
     * the library restores the topic name from the alias mapping and sets this flag to true.
     * @returns {boolean}
     */
    get topicNameExtracted() {
        const ret = wasm.wasmpublishpacketv5_0_topicNameExtracted(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns the user properties from the PUBLISH packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmpublishpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPublishPacketV5_0.prototype[Symbol.dispose] = WasmPublishPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 PUBREC packet
 */
export class WasmPubrecPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubrecPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubrecPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubrecPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubrecpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubrecpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubrecPacketV3_1_1.prototype[Symbol.dispose] = WasmPubrecPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 PUBREC packet
 */
export class WasmPubrecPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubrecPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubrecPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubrecPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubrecpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubrecpacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmpubrecpacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmpubrecpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the PUBREC packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmpubrecpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubrecPacketV5_0.prototype[Symbol.dispose] = WasmPubrecPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 PUBREL packet
 */
export class WasmPubrelPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubrelPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubrelPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubrelPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubrelpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubrelpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubrelPacketV3_1_1.prototype[Symbol.dispose] = WasmPubrelPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 PUBREL packet
 */
export class WasmPubrelPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPubrelPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmPubrelPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPubrelPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpubrelpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmpubrelpacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get reasonCode() {
        const ret = wasm.wasmpubcomppacketv5_0_reasonCode(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmpubrelpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the PUBREL packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmpubrelpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmPubrelPacketV5_0.prototype[Symbol.dispose] = WasmPubrelPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 SUBACK packet
 */
export class WasmSubackPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmSubackPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmSubackPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSubackPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmsubackpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmsubackpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Uint8Array}
     */
    returnCodes() {
        const ret = wasm.wasmsubackpacketv3_1_1_returnCodes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) WasmSubackPacketV3_1_1.prototype[Symbol.dispose] = WasmSubackPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 SUBACK packet
 */
export class WasmSubackPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmSubackPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmSubackPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSubackPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmsubackpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmsubackpacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Uint8Array}
     */
    reasonCodes() {
        const ret = wasm.wasmsubackpacketv5_0_reasonCodes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmsubackpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the SUBACK packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmsubackpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmSubackPacketV5_0.prototype[Symbol.dispose] = WasmSubackPacketV5_0.prototype.free;

/**
 * WASM wrapper for V3.1.1 UNSUBACK packet
 */
export class WasmUnsubackPacketV3_1_1 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmUnsubackPacketV3_1_1.prototype);
        obj.__wbg_ptr = ptr;
        WasmUnsubackPacketV3_1_1Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmUnsubackPacketV3_1_1Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmunsubackpacketv3_1_1_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmunsubackpacketv3_1_1_packetId(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmUnsubackPacketV3_1_1.prototype[Symbol.dispose] = WasmUnsubackPacketV3_1_1.prototype.free;

/**
 * WASM wrapper for V5.0 UNSUBACK packet
 */
export class WasmUnsubackPacketV5_0 {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmUnsubackPacketV5_0.prototype);
        obj.__wbg_ptr = ptr;
        WasmUnsubackPacketV5_0Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmUnsubackPacketV5_0Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmunsubackpacketv5_0_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get packetId() {
        const ret = wasm.wasmunsubackpacketv5_0_packetId(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Uint8Array}
     */
    reasonCodes() {
        const ret = wasm.wasmunsubackpacketv5_0_reasonCodes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string | undefined}
     */
    get reasonString() {
        const ret = wasm.wasmunsubackpacketv5_0_reasonString(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns the user properties from the UNSUBACK packet.
     * Returns an array of {key, value} objects.
     * @returns {any}
     */
    userProperties() {
        const ret = wasm.wasmunsubackpacketv5_0_userProperties(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) WasmUnsubackPacketV5_0.prototype[Symbol.dispose] = WasmUnsubackPacketV5_0.prototype.free;

/**
 * Check WASM version for debugging
 * @returns {string}
 */
export function check_version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.check_version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Create a WasmMqttClient with a JsTransport
 * This is a helper function that properly sets up the transport handle
 * @param {WasmMqttConfig} config
 * @param {JsTransport} transport
 * @returns {WasmMqttClient}
 */
export function createClientWithJsTransport(config, transport) {
    _assertClass(config, WasmMqttConfig);
    var ptr0 = config.__destroy_into_raw();
    _assertClass(transport, JsTransport);
    const ret = wasm.createClientWithJsTransport(ptr0, transport.__wbg_ptr);
    return WasmMqttClient.__wrap(ret);
}

export function init() {
    wasm.init();
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_Number_04624de7d0e8332d: function(arg0) {
            const ret = Number(arg0);
            return ret;
        },
        __wbg_String_8f0eb39a4a4c2f66: function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_47fa6863be6f2f25: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbg___wbindgen_is_function_0095a73b8b156f76: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_ac34f5003991759a: function(arg0) {
            const ret = arg0 === null;
            return ret;
        },
        __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_8ff4255516ccad3e: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72fb696202c56729: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_d9b87ff7982e3b21: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_4708e0c13bdc8e95: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_clearTimeout_3efe816196039f75: function(arg0) {
            clearTimeout(arg0);
        },
        __wbg_close_1d08eaf57ed325c0: function() { return handleError(function (arg0) {
            arg0.close();
        }, arguments); },
        __wbg_code_a552f1e91eda69b7: function(arg0) {
            const ret = arg0.code;
            return ret;
        },
        __wbg_data_5330da50312d0bc1: function(arg0) {
            const ret = arg0.data;
            return ret;
        },
        __wbg_done_57b39ecd9addfe81: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_error_7534b8e9a36f1ab4: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_get_9b94d73e6221f75c: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_b3ed3ad4be2bc8ac: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_with_ref_key_1dc361bd10053bfe: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_CloseEvent_583acc02217272d2: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CloseEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ErrorEvent_cd1bf636fceb3180: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ErrorEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_MessageEvent_1a6960e6b15377ad: function(arg0) {
            let result;
            try {
                result = arg0 instanceof MessageEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_9b9075935c74707c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isArray_d314bb98fcf08331: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_isSafeInteger_bfbc7332a9768d2a: function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        },
        __wbg_iterator_6ff6560ca1568e55: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_length_32ed9a279acd054c: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_35a7bace40f36eac: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_log_6b5ca2e6124b2808: function(arg0) {
            console.log(arg0);
        },
        __wbg_log_bfd3b20124d9da61: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
        __wbg_message_6de0e1db93388eee: function(arg0, arg1) {
            const ret = arg1.message;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_new_361308b2356cecd0: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_8a6f238a6ece86ea: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_b5d9e2fb389fef91: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen__convert__closures_____invoke__h15985c99b34d4441(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_new_dd2b680c8bf6ae29: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_no_args_1c7c842f08d00ebb: function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_with_str_sequence_b67b3919b8b11238: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new WebSocket(getStringFromWasm0(arg0, arg1), arg2);
            return ret;
        }, arguments); },
        __wbg_next_3482f54c49e8af19: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_next_418f80d8f5303233: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_onClose_7f353915c944f59b: function(arg0) {
            arg0.onClose();
        },
        __wbg_onSend_31326c9166c22ca0: function(arg0, arg1, arg2) {
            arg0.onSend(getArrayU8FromWasm0(arg1, arg2));
        },
        __wbg_prototypesetcall_bdcdcc5842e4d77d: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_push_8ffdcb2063340ba5: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_queueMicrotask_0aa0a927f78f5d98: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_queueMicrotask_5bb536982f78a56f: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_reason_35fce8e55dd90f31: function(arg0, arg1) {
            const ret = arg1.reason;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_resolve_002c4b7d9d8f6b64: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_send_542f95dea2df7994: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.send(getArrayU8FromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_setTimeout_01d27407edb60efa: function(arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        },
        __wbg_set_6cb8631f80447a67: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_binaryType_5bbf62e9f705dc1a: function(arg0, arg1) {
            arg0.binaryType = __wbindgen_enum_BinaryType[arg1];
        },
        __wbg_set_onclose_d382f3e2c2b850eb: function(arg0, arg1) {
            arg0.onclose = arg1;
        },
        __wbg_set_onerror_377f18bf4569bf85: function(arg0, arg1) {
            arg0.onerror = arg1;
        },
        __wbg_set_onmessage_2114aa5f4f53051e: function(arg0, arg1) {
            arg0.onmessage = arg1;
        },
        __wbg_set_onopen_b7b52d519d6c0f11: function(arg0, arg1) {
            arg0.onopen = arg1;
        },
        __wbg_stack_0ed75d68575b0f3c: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_static_accessor_GLOBAL_12837167ad935116: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_then_b9e7b3b5f1a9e1b5: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_value_0546255b415e96c1: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_wasClean_a9c77a7100d8534f: function(arg0) {
            const ret = arg0.wasClean;
            return ret;
        },
        __wbg_wasmauthpacketv5_0_new: function(arg0) {
            const ret = WasmAuthPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmconnackpacketv3_1_1_new: function(arg0) {
            const ret = WasmConnackPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmconnackpacketv5_0_new: function(arg0) {
            const ret = WasmConnackPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmdisconnectpacketv3_1_1_new: function(arg0) {
            const ret = WasmDisconnectPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmdisconnectpacketv5_0_new: function(arg0) {
            const ret = WasmDisconnectPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmmqttpacket_new: function(arg0) {
            const ret = WasmMqttPacket.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubackpacketv3_1_1_new: function(arg0) {
            const ret = WasmPubackPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubackpacketv5_0_new: function(arg0) {
            const ret = WasmPubackPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubcomppacketv3_1_1_new: function(arg0) {
            const ret = WasmPubcompPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubcomppacketv5_0_new: function(arg0) {
            const ret = WasmPubcompPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpublishpacketv3_1_1_new: function(arg0) {
            const ret = WasmPublishPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpublishpacketv5_0_new: function(arg0) {
            const ret = WasmPublishPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubrecpacketv3_1_1_new: function(arg0) {
            const ret = WasmPubrecPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubrecpacketv5_0_new: function(arg0) {
            const ret = WasmPubrecPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubrelpacketv3_1_1_new: function(arg0) {
            const ret = WasmPubrelPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmpubrelpacketv5_0_new: function(arg0) {
            const ret = WasmPubrelPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmsubackpacketv3_1_1_new: function(arg0) {
            const ret = WasmSubackPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmsubackpacketv5_0_new: function(arg0) {
            const ret = WasmSubackPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbg_wasmunsubackpacketv3_1_1_new: function(arg0) {
            const ret = WasmUnsubackPacketV3_1_1.__wrap(arg0);
            return ret;
        },
        __wbg_wasmunsubackpacketv5_0_new: function(arg0) {
            const ret = WasmUnsubackPacketV5_0.__wrap(arg0);
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 12, function: Function { arguments: [], shim_idx: 13, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h4306aca33429b4a6, wasm_bindgen__convert__closures_____invoke__h64f681a631cbf326);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 168, function: Function { arguments: [Externref], shim_idx: 169, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h2b6faffa85410744, wasm_bindgen__convert__closures_____invoke__h56fe60d6109a3bfd);
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./mqtt_client_wasm_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__h64f681a631cbf326(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h64f681a631cbf326(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h56fe60d6109a3bfd(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h56fe60d6109a3bfd(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h15985c99b34d4441(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h15985c99b34d4441(arg0, arg1, arg2, arg3);
}


const __wbindgen_enum_BinaryType = ["blob", "arraybuffer"];
const JsTransportFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jstransport_free(ptr >>> 0, 1));
const WasmAuthPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmauthpacketv5_0_free(ptr >>> 0, 1));
const WasmConnackPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmconnackpacketv3_1_1_free(ptr >>> 0, 1));
const WasmConnackPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmconnackpacketv5_0_free(ptr >>> 0, 1));
const WasmDisconnectPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdisconnectpacketv3_1_1_free(ptr >>> 0, 1));
const WasmDisconnectPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdisconnectpacketv5_0_free(ptr >>> 0, 1));
const WasmMqttClientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmqttclient_free(ptr >>> 0, 1));
const WasmMqttConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmqttconfig_free(ptr >>> 0, 1));
const WasmMqttPacketFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmqttpacket_free(ptr >>> 0, 1));
const WasmPubackPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubackpacketv3_1_1_free(ptr >>> 0, 1));
const WasmPubackPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubackpacketv5_0_free(ptr >>> 0, 1));
const WasmPubcompPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubcomppacketv3_1_1_free(ptr >>> 0, 1));
const WasmPubcompPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubcomppacketv5_0_free(ptr >>> 0, 1));
const WasmPublishPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpublishpacketv3_1_1_free(ptr >>> 0, 1));
const WasmPublishPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpublishpacketv5_0_free(ptr >>> 0, 1));
const WasmPubrecPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubrecpacketv3_1_1_free(ptr >>> 0, 1));
const WasmPubrecPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubrecpacketv5_0_free(ptr >>> 0, 1));
const WasmPubrelPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubrelpacketv3_1_1_free(ptr >>> 0, 1));
const WasmPubrelPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpubrelpacketv5_0_free(ptr >>> 0, 1));
const WasmSubackPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmsubackpacketv3_1_1_free(ptr >>> 0, 1));
const WasmSubackPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmsubackpacketv5_0_free(ptr >>> 0, 1));
const WasmUnsubackPacketV3_1_1Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmunsubackpacketv3_1_1_free(ptr >>> 0, 1));
const WasmUnsubackPacketV5_0Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmunsubackpacketv5_0_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('mqtt_client_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
