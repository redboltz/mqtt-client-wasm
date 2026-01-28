/* tslint:disable */
/* eslint-disable */

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
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new JavaScript transport bridge
     */
    constructor();
    /**
     * Called by JavaScript when the transport is closed
     */
    notifyClosed(): void;
    /**
     * Called by JavaScript when the transport connects successfully
     */
    notifyConnected(): void;
    /**
     * Called by JavaScript when an error occurs
     */
    notifyError(error: string): void;
    /**
     * Called by JavaScript when data is received from the transport
     */
    notifyMessage(data: Uint8Array): void;
    /**
     * Set the JavaScript callbacks for transport operations
     *
     * The callbacks object must implement:
     * - onSend(data: Uint8Array): void - called to send data via transport
     * - onClose(): void - called to close the transport
     */
    setCallbacks(callbacks: JsTransportCallbacks): void;
}

/**
 * WASM wrapper for V5.0 AUTH packet
 */
export class WasmAuthPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the AUTH packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly authenticationData: Uint8Array | undefined;
    readonly authenticationMethod: string | undefined;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 CONNACK packet
 */
export class WasmConnackPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    isSuccess(): boolean;
    readonly returnCode: number;
    readonly sessionPresent: boolean;
}

/**
 * WASM wrapper for V5.0 CONNACK packet
 */
export class WasmConnackPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    isSuccess(): boolean;
    /**
     * Returns the user properties from the CONNACK packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly assignedClientIdentifier: string | undefined;
    readonly authenticationData: Uint8Array | undefined;
    readonly authenticationMethod: string | undefined;
    readonly maximumPacketSize: number | undefined;
    readonly maximumQos: number | undefined;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
    readonly receiveMaximum: number | undefined;
    readonly responseInformation: string | undefined;
    readonly retainAvailable: boolean | undefined;
    readonly serverKeepAlive: number | undefined;
    readonly serverReference: string | undefined;
    readonly sessionExpiryInterval: number | undefined;
    readonly sessionPresent: boolean;
    readonly sharedSubscriptionAvailable: boolean | undefined;
    readonly subscriptionIdentifiersAvailable: boolean | undefined;
    readonly topicAliasMaximum: number | undefined;
    readonly wildcardSubscriptionAvailable: boolean | undefined;
}

/**
 * WASM wrapper for V3.1.1 DISCONNECT packet
 */
export class WasmDisconnectPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * WASM wrapper for V5.0 DISCONNECT packet
 */
export class WasmDisconnectPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the DISCONNECT packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
    readonly serverReference: string | undefined;
    readonly sessionExpiryInterval: number | undefined;
}

/**
 * WASM-friendly wrapper around MqttClient
 */
export class WasmMqttClient {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Acquire a packet ID
     */
    acquirePacketId(): Promise<number | undefined>;
    /**
     * Convert packet to AUTH wrapper (V5.0 only)
     * Returns WasmAuthPacketV5_0 for V5.0 clients, null otherwise
     */
    asAuth(packet: WasmMqttPacket): any;
    /**
     * Convert packet to CONNACK wrapper (version-aware)
     * Returns WasmConnackPacketV3_1_1 or WasmConnackPacketV5_0 based on client version
     */
    asConnack(packet: WasmMqttPacket): any;
    /**
     * Convert packet to DISCONNECT wrapper (version-aware)
     * Returns WasmDisconnectPacketV3_1_1 or WasmDisconnectPacketV5_0 based on client version
     */
    asDisconnect(packet: WasmMqttPacket): any;
    /**
     * Convert packet to PUBACK wrapper (version-aware)
     * Returns WasmPubackPacketV3_1_1 or WasmPubackPacketV5_0 based on client version
     */
    asPuback(packet: WasmMqttPacket): any;
    /**
     * Convert packet to PUBCOMP wrapper (version-aware)
     * Returns WasmPubcompPacketV3_1_1 or WasmPubcompPacketV5_0 based on client version
     */
    asPubcomp(packet: WasmMqttPacket): any;
    /**
     * Convert packet to PUBLISH wrapper (version-aware)
     * Returns WasmPublishPacketV3_1_1 or WasmPublishPacketV5_0 based on client version
     */
    asPublish(packet: WasmMqttPacket): any;
    /**
     * Convert packet to PUBREC wrapper (version-aware)
     * Returns WasmPubrecPacketV3_1_1 or WasmPubrecPacketV5_0 based on client version
     */
    asPubrec(packet: WasmMqttPacket): any;
    /**
     * Convert packet to PUBREL wrapper (version-aware)
     * Returns WasmPubrelPacketV3_1_1 or WasmPubrelPacketV5_0 based on client version
     */
    asPubrel(packet: WasmMqttPacket): any;
    /**
     * Convert packet to SUBACK wrapper (version-aware)
     * Returns WasmSubackPacketV3_1_1 or WasmSubackPacketV5_0 based on client version
     */
    asSuback(packet: WasmMqttPacket): any;
    /**
     * Convert packet to UNSUBACK wrapper (version-aware)
     * Returns WasmUnsubackPacketV3_1_1 or WasmUnsubackPacketV5_0 based on client version
     */
    asUnsuback(packet: WasmMqttPacket): any;
    /**
     * Close the connection
     */
    close(): Promise<void>;
    /**
     * Connect to MQTT broker
     */
    connect(url: string): Promise<void>;
    /**
     * Get connection state
     */
    isConnected(): Promise<boolean>;
    constructor(config: WasmMqttConfig);
    /**
     * Create Auth packet (V5.0 only)
     * Returns error if called on V3.1.1 client
     */
    newAuthPacket(options: any): WasmMqttPacket;
    /**
     * Create Connect packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     */
    newConnectPacket(options: any): WasmMqttPacket;
    /**
     * Create Disconnect packet (version-aware)
     * For V3.1.1: options are ignored (Disconnect has no fields)
     * For V5.0: reason_code, reason_string, session_expiry_interval, user_properties are used
     * Pass undefined/null for default options
     */
    newDisconnectPacket(options: any): WasmMqttPacket;
    /**
     * Create Pingreq packet (version-aware)
     */
    newPingreqPacket(): WasmMqttPacket;
    /**
     * Create Puback packet (version-aware)
     * For V3.1.1: only packet_id is used
     * For V5.0: packet_id, reason_code, reason_string, user_properties are used
     */
    newPubackPacket(options: any): WasmMqttPacket;
    /**
     * Create Pubcomp packet (version-aware)
     */
    newPubcompPacket(options: any): WasmMqttPacket;
    /**
     * Create Publish packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     */
    newPublishPacket(options: any): WasmMqttPacket;
    /**
     * Create Pubrec packet (version-aware)
     */
    newPubrecPacket(options: any): WasmMqttPacket;
    /**
     * Create Pubrel packet (version-aware)
     */
    newPubrelPacket(options: any): WasmMqttPacket;
    /**
     * Create Subscribe packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     */
    newSubscribePacket(options: any): WasmMqttPacket;
    /**
     * Create Unsubscribe packet (version-aware)
     * Automatically creates V3.1.1 or V5.0 packet based on client version
     */
    newUnsubscribePacket(options: any): WasmMqttPacket;
    /**
     * Receive next packet
     */
    recv(): Promise<WasmMqttPacket>;
    /**
     * Register a packet ID
     */
    registerPacketId(packet_id: number): Promise<boolean>;
    /**
     * Release a packet ID
     */
    releasePacketId(packet_id: number): Promise<void>;
    /**
     * Send MQTT packet
     */
    send(packet: WasmMqttPacket): Promise<void>;
}

/**
 * WASM-friendly wrapper around MqttConfig
 */
export class WasmMqttConfig {
    free(): void;
    [Symbol.dispose](): void;
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
     */
    constructor(options: any);
}

/**
 * WASM wrapper for MQTT packets
 */
export class WasmMqttPacket {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Parse packet from bytes
     * Note: This creates a temporary MQTT connection to use its parser
     */
    static fromBytes(data: Uint8Array, version: string): WasmMqttPacket;
    /**
     * Create V5.0 Auth packet from JSON options
     */
    static newAuthV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Connect packet from JSON options
     */
    static newConnectV311(options: any): WasmMqttPacket;
    /**
     * Create V5.0 Connect packet from JSON options
     */
    static newConnectV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Disconnect packet
     */
    static newDisconnectV311(): WasmMqttPacket;
    /**
     * Create V5.0 Disconnect packet from JSON options
     */
    static newDisconnectV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Pingreq packet
     */
    static newPingreqV311(): WasmMqttPacket;
    /**
     * Create V5.0 Pingreq packet
     */
    static newPingreqV50(): WasmMqttPacket;
    /**
     * Create V3.1.1 Puback packet
     */
    static newPubackV311(packet_id: number): WasmMqttPacket;
    /**
     * Create V5.0 Puback packet from JSON options
     */
    static newPubackV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Pubcomp packet
     */
    static newPubcompV311(packet_id: number): WasmMqttPacket;
    /**
     * Create V5.0 Pubcomp packet from JSON options
     */
    static newPubcompV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Publish packet from JSON options
     */
    static newPublishV311(options: any): WasmMqttPacket;
    /**
     * Create V5.0 Publish packet from JSON options
     */
    static newPublishV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Pubrec packet
     */
    static newPubrecV311(packet_id: number): WasmMqttPacket;
    /**
     * Create V5.0 Pubrec packet from JSON options
     */
    static newPubrecV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Pubrel packet
     */
    static newPubrelV311(packet_id: number): WasmMqttPacket;
    /**
     * Create V5.0 Pubrel packet from JSON options
     */
    static newPubrelV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Subscribe packet from JSON options
     */
    static newSubscribeV311(options: any): WasmMqttPacket;
    /**
     * Create V5.0 Subscribe packet from JSON options
     */
    static newSubscribeV50(options: any): WasmMqttPacket;
    /**
     * Create V3.1.1 Unsubscribe packet from JSON options
     */
    static newUnsubscribeV311(options: any): WasmMqttPacket;
    /**
     * Create V5.0 Unsubscribe packet from JSON options
     */
    static newUnsubscribeV50(options: any): WasmMqttPacket;
    /**
     * Get packet type as enum
     */
    packetType(): WasmPacketType;
    /**
     * Get packet type as string (for debugging)
     */
    packetTypeString(): string;
    /**
     * Serialize packet to bytes
     */
    toBytes(): Uint8Array;
}

/**
 * Packet type enum exposed to JavaScript
 */
export enum WasmPacketType {
    Connect = 0,
    Connack = 1,
    Publish = 2,
    Puback = 3,
    Pubrec = 4,
    Pubrel = 5,
    Pubcomp = 6,
    Subscribe = 7,
    Suback = 8,
    Unsubscribe = 9,
    Unsuback = 10,
    Pingreq = 11,
    Pingresp = 12,
    Disconnect = 13,
    Auth = 14,
}

/**
 * WASM wrapper for V3.1.1 PUBACK packet
 */
export class WasmPubackPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 PUBACK packet
 */
export class WasmPubackPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the PUBACK packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 PUBCOMP packet
 */
export class WasmPubcompPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 PUBCOMP packet
 */
export class WasmPubcompPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the PUBCOMP packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 PUBLISH packet
 */
export class WasmPublishPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    payloadBytes(): Uint8Array;
    readonly dup: boolean;
    readonly packetId: number | undefined;
    readonly payload: string | undefined;
    readonly qos: number;
    readonly retain: boolean;
    readonly topicName: string;
}

/**
 * WASM wrapper for V5.0 PUBLISH packet
 */
export class WasmPublishPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    payloadBytes(): Uint8Array;
    /**
     * Returns the subscription identifiers from the PUBLISH packet.
     * These are set by the broker to indicate which subscriptions matched the message.
     * Multiple subscription identifiers can be present if the message matches multiple subscriptions.
     */
    subscriptionIdentifiers(): Uint32Array;
    /**
     * Returns the user properties from the PUBLISH packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly contentType: string | undefined;
    readonly correlationData: Uint8Array | undefined;
    readonly dup: boolean;
    readonly messageExpiryInterval: number | undefined;
    readonly packetId: number | undefined;
    readonly payload: string | undefined;
    readonly payloadFormatIndicator: number | undefined;
    readonly qos: number;
    readonly responseTopic: string | undefined;
    readonly retain: boolean;
    readonly topicAlias: number | undefined;
    readonly topicName: string;
    /**
     * Returns true if the topic name was extracted from topic alias mapping.
     * When a PUBLISH packet is received with an empty topic name and a topic alias,
     * the library restores the topic name from the alias mapping and sets this flag to true.
     */
    readonly topicNameExtracted: boolean;
}

/**
 * WASM wrapper for V3.1.1 PUBREC packet
 */
export class WasmPubrecPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 PUBREC packet
 */
export class WasmPubrecPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the PUBREC packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 PUBREL packet
 */
export class WasmPubrelPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 PUBREL packet
 */
export class WasmPubrelPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the user properties from the PUBREL packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonCode: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 SUBACK packet
 */
export class WasmSubackPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    returnCodes(): Uint8Array;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 SUBACK packet
 */
export class WasmSubackPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    reasonCodes(): Uint8Array;
    /**
     * Returns the user properties from the SUBACK packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonString: string | undefined;
}

/**
 * WASM wrapper for V3.1.1 UNSUBACK packet
 */
export class WasmUnsubackPacketV3_1_1 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly packetId: number;
}

/**
 * WASM wrapper for V5.0 UNSUBACK packet
 */
export class WasmUnsubackPacketV5_0 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    reasonCodes(): Uint8Array;
    /**
     * Returns the user properties from the UNSUBACK packet.
     * Returns an array of {key, value} objects.
     */
    userProperties(): any;
    readonly packetId: number;
    readonly reasonString: string | undefined;
}

/**
 * Check WASM version for debugging
 */
export function check_version(): string;

/**
 * Create a WasmMqttClient with a JsTransport
 * This is a helper function that properly sets up the transport handle
 */
export function createClientWithJsTransport(config: WasmMqttConfig, transport: JsTransport): WasmMqttClient;

export function init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmauthpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmconnackpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmconnackpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmdisconnectpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmdisconnectpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmmqttclient_free: (a: number, b: number) => void;
    readonly __wbg_wasmmqttconfig_free: (a: number, b: number) => void;
    readonly __wbg_wasmmqttpacket_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubackpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubackpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubcomppacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubcomppacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmpublishpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmpublishpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubrecpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubrecpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubrelpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmpubrelpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmsubackpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmsubackpacketv5_0_free: (a: number, b: number) => void;
    readonly __wbg_wasmunsubackpacketv3_1_1_free: (a: number, b: number) => void;
    readonly __wbg_wasmunsubackpacketv5_0_free: (a: number, b: number) => void;
    readonly check_version: () => [number, number];
    readonly wasmauthpacketv5_0_authenticationData: (a: number) => [number, number];
    readonly wasmauthpacketv5_0_authenticationMethod: (a: number) => [number, number];
    readonly wasmauthpacketv5_0_reasonCode: (a: number) => number;
    readonly wasmauthpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmauthpacketv5_0_userProperties: (a: number) => any;
    readonly wasmconnackpacketv3_1_1_isSuccess: (a: number) => number;
    readonly wasmconnackpacketv3_1_1_returnCode: (a: number) => number;
    readonly wasmconnackpacketv3_1_1_sessionPresent: (a: number) => number;
    readonly wasmconnackpacketv5_0_assignedClientIdentifier: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_authenticationData: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_authenticationMethod: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_isSuccess: (a: number) => number;
    readonly wasmconnackpacketv5_0_maximumPacketSize: (a: number) => number;
    readonly wasmconnackpacketv5_0_maximumQos: (a: number) => number;
    readonly wasmconnackpacketv5_0_reasonCode: (a: number) => number;
    readonly wasmconnackpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_receiveMaximum: (a: number) => number;
    readonly wasmconnackpacketv5_0_responseInformation: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_retainAvailable: (a: number) => number;
    readonly wasmconnackpacketv5_0_serverKeepAlive: (a: number) => number;
    readonly wasmconnackpacketv5_0_serverReference: (a: number) => [number, number];
    readonly wasmconnackpacketv5_0_sessionExpiryInterval: (a: number) => number;
    readonly wasmconnackpacketv5_0_sessionPresent: (a: number) => number;
    readonly wasmconnackpacketv5_0_sharedSubscriptionAvailable: (a: number) => number;
    readonly wasmconnackpacketv5_0_subscriptionIdentifiersAvailable: (a: number) => number;
    readonly wasmconnackpacketv5_0_topicAliasMaximum: (a: number) => number;
    readonly wasmconnackpacketv5_0_userProperties: (a: number) => any;
    readonly wasmconnackpacketv5_0_wildcardSubscriptionAvailable: (a: number) => number;
    readonly wasmdisconnectpacketv5_0_reasonCode: (a: number) => number;
    readonly wasmdisconnectpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmdisconnectpacketv5_0_serverReference: (a: number) => [number, number];
    readonly wasmdisconnectpacketv5_0_sessionExpiryInterval: (a: number) => number;
    readonly wasmdisconnectpacketv5_0_userProperties: (a: number) => any;
    readonly wasmmqttclient_acquirePacketId: (a: number) => any;
    readonly wasmmqttclient_asAuth: (a: number, b: number) => any;
    readonly wasmmqttclient_asConnack: (a: number, b: number) => any;
    readonly wasmmqttclient_asDisconnect: (a: number, b: number) => any;
    readonly wasmmqttclient_asPuback: (a: number, b: number) => any;
    readonly wasmmqttclient_asPubcomp: (a: number, b: number) => any;
    readonly wasmmqttclient_asPublish: (a: number, b: number) => any;
    readonly wasmmqttclient_asPubrec: (a: number, b: number) => any;
    readonly wasmmqttclient_asPubrel: (a: number, b: number) => any;
    readonly wasmmqttclient_asSuback: (a: number, b: number) => any;
    readonly wasmmqttclient_asUnsuback: (a: number, b: number) => any;
    readonly wasmmqttclient_close: (a: number) => any;
    readonly wasmmqttclient_connect: (a: number, b: number, c: number) => any;
    readonly wasmmqttclient_isConnected: (a: number) => any;
    readonly wasmmqttclient_new: (a: number) => number;
    readonly wasmmqttclient_newAuthPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newConnectPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newDisconnectPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newPingreqPacket: (a: number) => number;
    readonly wasmmqttclient_newPubackPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newPubcompPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newPublishPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newPubrecPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newPubrelPacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newSubscribePacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_newUnsubscribePacket: (a: number, b: any) => [number, number, number];
    readonly wasmmqttclient_recv: (a: number) => any;
    readonly wasmmqttclient_registerPacketId: (a: number, b: number) => any;
    readonly wasmmqttclient_releasePacketId: (a: number, b: number) => any;
    readonly wasmmqttclient_send: (a: number, b: number) => any;
    readonly wasmmqttconfig_new: (a: any) => [number, number, number];
    readonly wasmmqttpacket_fromBytes: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly wasmmqttpacket_newAuthV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newConnectV311: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newConnectV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newDisconnectV311: () => number;
    readonly wasmmqttpacket_newDisconnectV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPingreqV311: () => number;
    readonly wasmmqttpacket_newPingreqV50: () => number;
    readonly wasmmqttpacket_newPubackV311: (a: number) => [number, number, number];
    readonly wasmmqttpacket_newPubackV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPubcompV311: (a: number) => [number, number, number];
    readonly wasmmqttpacket_newPubcompV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPublishV311: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPublishV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPubrecV311: (a: number) => [number, number, number];
    readonly wasmmqttpacket_newPubrecV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newPubrelV311: (a: number) => [number, number, number];
    readonly wasmmqttpacket_newPubrelV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newSubscribeV311: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newSubscribeV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newUnsubscribeV311: (a: any) => [number, number, number];
    readonly wasmmqttpacket_newUnsubscribeV50: (a: any) => [number, number, number];
    readonly wasmmqttpacket_packetType: (a: number) => number;
    readonly wasmmqttpacket_packetTypeString: (a: number) => [number, number];
    readonly wasmmqttpacket_toBytes: (a: number) => [number, number];
    readonly wasmpubackpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmpubackpacketv5_0_packetId: (a: number) => number;
    readonly wasmpubackpacketv5_0_reasonCode: (a: number) => number;
    readonly wasmpubackpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmpubackpacketv5_0_userProperties: (a: number) => any;
    readonly wasmpubcomppacketv3_1_1_packetId: (a: number) => number;
    readonly wasmpubcomppacketv5_0_packetId: (a: number) => number;
    readonly wasmpubcomppacketv5_0_reasonCode: (a: number) => number;
    readonly wasmpubcomppacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmpubcomppacketv5_0_userProperties: (a: number) => any;
    readonly wasmpublishpacketv3_1_1_dup: (a: number) => number;
    readonly wasmpublishpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmpublishpacketv3_1_1_payload: (a: number) => [number, number];
    readonly wasmpublishpacketv3_1_1_payloadBytes: (a: number) => [number, number];
    readonly wasmpublishpacketv3_1_1_qos: (a: number) => number;
    readonly wasmpublishpacketv3_1_1_retain: (a: number) => number;
    readonly wasmpublishpacketv3_1_1_topicName: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_contentType: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_correlationData: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_dup: (a: number) => number;
    readonly wasmpublishpacketv5_0_messageExpiryInterval: (a: number) => number;
    readonly wasmpublishpacketv5_0_packetId: (a: number) => number;
    readonly wasmpublishpacketv5_0_payload: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_payloadBytes: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_payloadFormatIndicator: (a: number) => number;
    readonly wasmpublishpacketv5_0_qos: (a: number) => number;
    readonly wasmpublishpacketv5_0_responseTopic: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_retain: (a: number) => number;
    readonly wasmpublishpacketv5_0_subscriptionIdentifiers: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_topicAlias: (a: number) => number;
    readonly wasmpublishpacketv5_0_topicName: (a: number) => [number, number];
    readonly wasmpublishpacketv5_0_topicNameExtracted: (a: number) => number;
    readonly wasmpublishpacketv5_0_userProperties: (a: number) => any;
    readonly wasmpubrecpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmpubrecpacketv5_0_packetId: (a: number) => number;
    readonly wasmpubrecpacketv5_0_reasonCode: (a: number) => number;
    readonly wasmpubrecpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmpubrecpacketv5_0_userProperties: (a: number) => any;
    readonly wasmpubrelpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmpubrelpacketv5_0_packetId: (a: number) => number;
    readonly wasmpubrelpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmpubrelpacketv5_0_userProperties: (a: number) => any;
    readonly wasmsubackpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmsubackpacketv3_1_1_returnCodes: (a: number) => [number, number];
    readonly wasmsubackpacketv5_0_packetId: (a: number) => number;
    readonly wasmsubackpacketv5_0_reasonCodes: (a: number) => [number, number];
    readonly wasmsubackpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmsubackpacketv5_0_userProperties: (a: number) => any;
    readonly wasmunsubackpacketv3_1_1_packetId: (a: number) => number;
    readonly wasmunsubackpacketv5_0_packetId: (a: number) => number;
    readonly wasmunsubackpacketv5_0_reasonCodes: (a: number) => [number, number];
    readonly wasmunsubackpacketv5_0_reasonString: (a: number) => [number, number];
    readonly wasmunsubackpacketv5_0_userProperties: (a: number) => any;
    readonly wasmpubrelpacketv5_0_reasonCode: (a: number) => number;
    readonly __wbg_jstransport_free: (a: number, b: number) => void;
    readonly createClientWithJsTransport: (a: number, b: number) => number;
    readonly jstransport_new: () => number;
    readonly jstransport_notifyClosed: (a: number) => void;
    readonly jstransport_notifyConnected: (a: number) => void;
    readonly jstransport_notifyError: (a: number, b: number, c: number) => void;
    readonly jstransport_notifyMessage: (a: number, b: number, c: number) => void;
    readonly jstransport_setCallbacks: (a: number, b: any) => void;
    readonly init: () => void;
    readonly wasm_bindgen__closure__destroy__h4306aca33429b4a6: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h2b6faffa85410744: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h15985c99b34d4441: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h56fe60d6109a3bfd: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h64f681a631cbf326: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
