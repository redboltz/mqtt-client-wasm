# mqtt-client-wasm

[![CI](https://github.com/redboltz/mqtt-client-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/redboltz/mqtt-client-wasm/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/redboltz/mqtt-client-wasm/branch/main/graph/badge.svg)](https://codecov.io/gh/redboltz/mqtt-client-wasm)

MQTT client library compiled to WebAssembly, supporting both browsers and Node.js.

## Supported Transports

| Transport | Browser | Node.js |
|-----------|---------|---------|
| WebSocket (ws://) | Yes | Yes |
| WebSocket Secure (wss://) | Yes | Yes |
| TCP | - | Yes |
| TLS | - | Yes |

## Features

- **MQTT v3.1.1 and v5.0**: Full support for both protocol versions
- **Multiple Transports**: WebSocket for both platforms, TCP/TLS for Node.js
- **Protocol State Machine**: Internal state machine automatically handles protocol behavior based on packet exchange
  - e.g., KeepAlive in CONNECT triggers automatic PINGREQ transmission
  - e.g., MQTT v5.0 properties (receiveMaximum, maximumPacketSize, topicAliasMaximum, etc.) are internally tracked and enforced
  - See [mqtt-protocol-core](https://github.com/redboltz/mqtt-protocol-core) for details (Rust)
- **Low-level Endpoint API**: Direct packet send/recv operations for full control
- **Auto Response Options**: Configurable automatic handling of PUBACK, PUBREC, PUBREL, PUBCOMP, and PINGRESP
- **Interactive Client Tool**: Ready-to-use HTML client for testing and debugging (browser)

### Protocol State Machine Details

#### MQTT v3.1.1

**Sending CONNECT Packet:**
- If `keepAlive` is set to a value greater than 0, automatic PINGREQ transmission is enabled. A PINGREQ packet is sent when no other packet has been transmitted within `keepAlive` seconds.
- If `cleanSession` is set to `true`, the endpoint's Session State is cleared.

**Receiving CONNACK Packet:**
- If `sessionPresent` is `false`, the endpoint's Session State is cleared. However, the session state configuration is retained, so subsequent PUBLISH packets with QoS 1 or QoS 2 will be stored.
- If `sessionPresent` is `true`, any stored PUBLISH and PUBREL packets are retransmitted.

#### MQTT v5.0

**Sending CONNECT Packet:**
- If `cleanStart` is set to `true`, the endpoint's Session State is cleared.
- If `topicAliasMaximum` is set, a topic name to topic alias mapping is prepared for outgoing packets.
- If `receiveMaximum` is set, the receive quota for incoming packets is configured.
- If `maximumPacketSize` is set, the maximum packet size for incoming packets is configured.
- If `sessionExpiryInterval` is set to a value greater than 0, session state persistence is enabled.

**Receiving CONNACK Packet:**
- If `sessionPresent` is `false`, the endpoint's Session State is cleared. However, the session state configuration is retained, so subsequent PUBLISH packets with QoS 1 or QoS 2 will be stored.
- If `sessionPresent` is `true`, any stored PUBLISH and PUBREL packets are retransmitted.
- If `topicAliasMaximum` is set, a topic alias to topic name mapping is prepared for incoming packets.
- If `receiveMaximum` is set, the send quota for outgoing packets is configured.
- If `maximumPacketSize` is set, the maximum packet size for outgoing packets is configured.
- If `serverKeepAlive` is set, it overrides the client's `keepAlive` value. A PINGREQ packet is sent when no other packet has been transmitted within `serverKeepAlive` seconds.

#### Topic Alias

Topic Alias is an MQTT v5.0 feature that reduces PUBLISH packet size by mapping topic names to numeric identifiers. The mapping operates independently in two directions: broker-to-client and client-to-broker.

**Capacity Negotiation:**
- **Client to Broker:** The broker declares a `topicAliasMaximum` value in the CONNACK packet, allowing the client to use topic aliases up to that limit when sending PUBLISH packets.
- **Broker to Client:** The client declares a `topicAliasMaximum` value in the CONNECT packet, allowing the broker to use topic aliases up to that limit when sending PUBLISH packets.

**How Topic Alias Works:**

1. **Registering a mapping:** A PUBLISH packet containing both a topic name and a `topicAlias` property establishes the mapping. If the alias was already mapped to a different topic, the mapping is updated.

2. **Using a mapping:** A PUBLISH packet with an empty topic name and a `topicAlias` property signals the receiver to look up the topic name from the established mapping. This reduces packet size, especially for long topic names.

**Automatic Topic Alias Handling:**

The client provides two configuration options to automate topic alias management:

- `autoMapTopicAliasSend: true` - Automatically assigns topic aliases when sending PUBLISH packets. When all available aliases are in use, the least recently used mapping is replaced.

- `autoReplaceTopicAliasSend: true` - Automatically uses existing mappings when sending PUBLISH packets. If a topic name already has an alias registered, the client sends an empty topic name with the alias instead.

These options can be used together for fully automatic topic alias management.

**Receiving PUBLISH with Topic Alias:**

When the client receives a PUBLISH packet with an empty topic name and a topic alias, the library automatically restores the topic name from the mapping. You can check whether the topic name was restored by reading the `topicNameExtracted` property:

```javascript
const pub = client.asPublish(packet);
console.log(`Topic: ${pub.topicName}`);
if (pub.topicNameExtracted) {
    console.log('(topic name was restored from alias mapping)');
}
```

**Important: Topic Alias and Reconnection**

According to the MQTT specification, topic alias mappings are **not** part of Session State. This means all mappings are discarded when the connection closes. However, Session State (which includes unacknowledged QoS 1/2 PUBLISH packets) can persist across connections.

This creates a potential issue: if a QoS 1 or QoS 2 PUBLISH packet was sent with an empty topic name (using a topic alias), and the client reconnects before receiving acknowledgment, the stored packet cannot be resent as-is because the alias mapping no longer exists.

**The library handles this automatically:** When retransmitting stored packets after reconnection, the library reconstructs each packet with the original topic name and removes the topic alias property. This ensures protocol compliance without requiring any user intervention.

---

## Installation

### npm

```bash
npm install @redboltz/mqtt-client-wasm
```

For WebSocket transport in Node.js, also install the `ws` package:

```bash
npm install ws
```

---

## Setup by Platform

### Browser Setup

```javascript
import init, {
    WasmMqttClient,
    WasmMqttConfig,
    WasmPacketType
} from '@redboltz/mqtt-client-wasm';

// Initialize WASM module (required once)
await init();

// Create client
const config = new WasmMqttConfig({
    version: '5.0',           // '3.1.1' or '5.0'
    autoPubResponse: true,    // Auto handle QoS acknowledgments
    autoPingResponse: true,   // Auto respond to PINGREQ
});
const client = new WasmMqttClient(config);

// Connect via WebSocket
await client.connect('wss://broker.example.com:8884/');
```

#### Direct Script Include (No Bundler)

```html
<script type="module">
    import init, { WasmMqttClient, WasmMqttConfig } from './pkg/mqtt_client_wasm.js';
    await init();
    // ... use the client
</script>
```

### Node.js Setup

```javascript
const {
    WasmMqttClient,
    WasmMqttConfig,
    WasmMqttPacket,
    WasmPacketType,
    init,
    createClientWithTransport,
    NodeWebSocketTransport,
    NodeTcpTransport,
    NodeTlsTransport
} = require('@redboltz/mqtt-client-wasm');

// Initialize WASM module (required once)
init();
```

#### Usage

Use `createClientWithTransport()` to create a client with a transport. The WASM client handles state machine, timers, and automatic responses (same API as browser).

```javascript
const fs = require('fs');

// Create transport and client
const transport = new NodeTcpTransport();
const config = new WasmMqttConfig({ version: '5.0' });
const client = createClientWithTransport(config, transport);

// Connect transport AFTER creating client
await transport.connect('broker.example.com', 1883);

// Use the same API as browser
const connectPacket = client.newConnectPacket({
    clientId: 'my-node-client',
    keepAlive: 60,
    cleanSession: true,
});
await client.send(connectPacket);

const connack = await client.recv();
console.log('Connected:', client.asConnack(connack).sessionPresent);

// Subscribe
const packetId = await client.acquirePacketId();
const subPacket = client.newSubscribePacket({
    packetId,
    subscriptions: [{ topic: 'test/#', qos: 1 }],
});
await client.send(subPacket);
const suback = await client.recv();

// Receive messages
while (true) {
    const packet = await client.recv();
    if (packet.packetType() === WasmPacketType.Publish) {
        const pub = client.asPublish(packet);
        console.log(`Received: ${pub.topicName} = ${pub.payload}`);
    }
}
```

#### Transport Types

**TCP Transport:**
```javascript
const transport = new NodeTcpTransport();
await transport.connect('broker.example.com', 1883);
```

**TLS Transport:**
```javascript
const transport = new NodeTlsTransport({
    ca: fs.readFileSync('ca.pem'),
});
await transport.connect('broker.example.com', 8883);
```

**WebSocket Transport (ws://):**
```javascript
const transport = new NodeWebSocketTransport();
await transport.connect('ws://broker.example.com:8080/');
```

**WebSocket Secure Transport (wss://):**
```javascript
const transport = new NodeWebSocketTransport({
    ca: fs.readFileSync('ca.pem'),
});
await transport.connect('wss://broker.example.com:8884/');
```

#### TLS Options (for NodeTlsTransport and NodeWebSocketTransport)

Both `NodeTlsTransport` and `NodeWebSocketTransport` accept a TLS options object in their constructor. These options are passed directly to Node.js TLS module.

| Option | Type | Description |
|--------|------|-------------|
| `ca` | Buffer/string | CA certificate(s) for server verification |
| `cert` | Buffer/string | Client certificate for mutual TLS |
| `key` | Buffer/string | Client private key for mutual TLS |
| `passphrase` | string | Passphrase for encrypted private key |
| `rejectUnauthorized` | boolean | Reject connections with unverified certificates (default: true) |
| `servername` | string | Server name for SNI (Server Name Indication) |
| `minVersion` | string | Minimum TLS version ('TLSv1.2', 'TLSv1.3') |
| `maxVersion` | string | Maximum TLS version |
| `ciphers` | string | Cipher suites to use |

**Example: Mutual TLS Authentication**

```javascript
const transport = new NodeTlsTransport({
    ca: fs.readFileSync('ca.pem'),
    cert: fs.readFileSync('client.crt'),
    key: fs.readFileSync('client.key'),
    passphrase: 'optional-key-passphrase',
});
```

**Example: Custom TLS Configuration**

```javascript
const transport = new NodeWebSocketTransport({
    ca: fs.readFileSync('ca.pem'),
    minVersion: 'TLSv1.2',
    servername: 'broker.example.com',
});
```

---

## Common API (Both Platforms)

The following sections apply to both browser and Node.js environments.

### Configuration Options

```javascript
const config = new WasmMqttConfig({
    version: '5.0',
    autoPubResponse: true,
    autoPingResponse: true,
});
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `'3.1.1'` | MQTT version (`'3.1.1'` or `'5.0'`) |
| `pingreqSendIntervalMs` | number | (auto) | Ping interval in ms (omit for auto from keepAlive) |
| `autoPubResponse` | boolean | `true` | Auto handle QoS acknowledgments |
| `autoPingResponse` | boolean | `true` | Auto respond to PINGREQ |
| `autoMapTopicAliasSend` | boolean | `false` | Auto map topic aliases (v5.0) |
| `autoReplaceTopicAliasSend` | boolean | `false` | Auto replace topic with alias (v5.0) |
| `pingrespRecvTimeoutMs` | number | (disabled) | PINGRESP timeout in ms |
| `connectionEstablishTimeoutMs` | number | (disabled) | Connection timeout in ms |
| `shutdownTimeoutMs` | number | (disabled) | Shutdown timeout in ms |

---

## Packet Reference

### Connect

```javascript
const connectPacket = client.newConnectPacket({
    clientId: 'my-client',
    keepAlive: 60,
    cleanSession: true,
    userName: 'user',
    password: 'pass',
    // Will message
    willTopic: 'client/status',
    willPayload: 'offline',
    willQos: 1,
    willRetain: true,
    // v5.0 Properties
    sessionExpiryInterval: 3600,
    receiveMaximum: 65535,
    maximumPacketSize: 1048576,
    topicAliasMaximum: 10,
    requestResponseInformation: true,
    requestProblemInformation: true,
    userProperties: [{ key: 'app', value: 'myapp' }],
    authenticationMethod: 'SCRAM-SHA-256',
    authenticationData: [0x01, 0x02, 0x03],
});

await client.send(connectPacket);
```

#### Connect Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `clientId` | string | Yes | Client identifier |
| `keepAlive` | number | No | Keep alive interval in seconds (default: 0) |
| `cleanSession` | boolean | No | Clean session (v3.1.1) / Clean start (v5.0) |
| `userName` | string | No | Username for authentication |
| `password` | string | No | Password for authentication |
| `willTopic` | string | No | Will message topic |
| `willPayload` | string | No | Will message payload |
| `willQos` | number | No | Will message QoS (0, 1, 2) |
| `willRetain` | boolean | No | Will message retain flag |

#### Connect Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `sessionExpiryInterval` | number | Session expiry interval in seconds |
| `receiveMaximum` | number | Maximum concurrent QoS 1/2 messages |
| `maximumPacketSize` | number | Maximum packet size in bytes |
| `topicAliasMaximum` | number | Maximum topic aliases |
| `requestResponseInformation` | boolean | Request response information from broker |
| `requestProblemInformation` | boolean | Request problem information from broker |
| `userProperties` | array | User properties `[{key, value}, ...]` |
| `authenticationMethod` | string | Authentication method name |
| `authenticationData` | array | Authentication data (byte array) |

---

### Subscribe

```javascript
const packetId = await client.acquirePacketId();
const subscribePacket = client.newSubscribePacket({
    packetId: packetId,
    subscriptions: [{
        topic: 'sensor/#',
        qos: 2,
        // v5.0 options
        noLocal: true,
        retainAsPublished: true,
        retainHandling: 1,
    }],
    // v5.0 Properties
    subscriptionIdentifier: 12345,
    userProperties: [{ key: 'source', value: 'web' }],
});

await client.send(subscribePacket);
```

#### Subscribe Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `packetId` | number | Yes | Packet identifier |
| `subscriptions` | array | Yes | Array of subscription entries |

#### Subscription Entry

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `topic` | string | Yes | Topic filter |
| `qos` | number | No | Maximum QoS (0, 1, 2, default: 0) |
| `noLocal` | boolean | No | (v5.0) Don't receive own messages |
| `retainAsPublished` | boolean | No | (v5.0) Keep retain flag as published |
| `retainHandling` | number | No | (v5.0) Retain handling: 0=send, 1=send if new, 2=don't send |

#### Subscribe Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `subscriptionIdentifier` | number | Subscription identifier (1-268435455) |
| `userProperties` | array | User properties `[{key, value}, ...]` |

---

### Unsubscribe

```javascript
const packetId = await client.acquirePacketId();
const unsubscribePacket = client.newUnsubscribePacket({
    packetId: packetId,
    topics: ['sensor/#', 'device/+/status'],
    // v5.0 Properties
    userProperties: [{ key: 'reason', value: 'cleanup' }],
});

await client.send(unsubscribePacket);
```

#### Unsubscribe Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `packetId` | number | Yes | Packet identifier |
| `topics` | array | Yes | Array of topic filters to unsubscribe |

#### Unsubscribe Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `userProperties` | array | User properties `[{key, value}, ...]` |

---

### Publish

```javascript
const publishPacket = client.newPublishPacket({
    topicName: 'sensor/temperature',
    payload: '25.5',
    qos: 1,
    retain: false,
    dup: false,
    packetId: await client.acquirePacketId(),  // Required for QoS > 0
    // v5.0 Properties
    payloadFormatIndicator: 1,
    messageExpiryInterval: 3600,
    topicAlias: 1,
    responseTopic: 'response/sensor',
    correlationData: [0x01, 0x02, 0x03],
    contentType: 'application/json',
    userProperties: [{ key: 'unit', value: 'celsius' }],
});

await client.send(publishPacket);
```

#### Publish Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `topicName` | string | Yes | Topic name |
| `payload` | string | No | Payload as UTF-8 string |
| `payloadBytes` | array | No | Payload as byte array (takes precedence over `payload`) |
| `qos` | number | No | QoS level (0, 1, 2, default: 0) |
| `retain` | boolean | No | Retain flag |
| `dup` | boolean | No | Duplicate flag |
| `packetId` | number | No* | Packet identifier (*Required for QoS 1 or 2) |

#### Publish Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `payloadFormatIndicator` | number | Payload format: 0=unspecified, 1=UTF-8 |
| `messageExpiryInterval` | number | Message expiry interval in seconds |
| `topicAlias` | number | Topic alias (1-65535) |
| `responseTopic` | string | Response topic for request/response |
| `correlationData` | array | Correlation data (byte array) |
| `contentType` | string | Content type (MIME type) |
| `userProperties` | array | User properties `[{key, value}, ...]` |

---

### Disconnect

```javascript
const disconnectPacket = client.newDisconnectPacket({
    // v5.0 only options
    reasonCode: 0,
    reasonString: 'Normal disconnection',
    sessionExpiryInterval: 0,
    userProperties: [{ key: 'client', value: 'web' }],
});

await client.send(disconnectPacket);
```

#### Disconnect Options (v5.0 only)

| Option | Type | Description |
|--------|------|-------------|
| `reasonCode` | number | Disconnect reason code (default: 0 = Normal) |
| `reasonString` | string | Human-readable reason string |
| `sessionExpiryInterval` | number | Override session expiry interval |
| `userProperties` | array | User properties `[{key, value}, ...]` |

**Note:** v3.1.1 DISCONNECT has no options - just call `client.newDisconnectPacket({})`.

---

### QoS Response Packets (PUBACK, PUBREC, PUBREL, PUBCOMP)

When `autoPubResponse: false`, you need to manually send QoS responses:

```javascript
// v3.1.1 - only packet_id
const pubackPacket = client.newPubackPacket({ packetId: 1 });

// v5.0 - with optional properties
const pubackPacket = client.newPubackPacket({
    packetId: 1,
    reasonCode: 0,
    reasonString: 'Success',
    userProperties: [{ key: 'info', value: 'processed' }],
});
```

#### QoS Response Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `packetId` | number | Yes | Packet identifier |

#### QoS Response Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `reasonCode` | number | Reason code |
| `reasonString` | string | Human-readable reason |
| `userProperties` | array | User properties `[{key, value}, ...]` |

Available methods:
- `newPubackPacket(options)` - QoS 1 acknowledgment
- `newPubrecPacket(options)` - QoS 2 received
- `newPubrelPacket(options)` - QoS 2 release
- `newPubcompPacket(options)` - QoS 2 complete

---

### Auth (v5.0 only)

```javascript
const authPacket = client.newAuthPacket({
    reasonCode: 0x18,  // Continue authentication
    authenticationMethod: 'SCRAM-SHA-256',
    authenticationData: [0x01, 0x02, 0x03],
    reasonString: 'Continue',
    userProperties: [{ key: 'step', value: '2' }],
});

await client.send(authPacket);
```

#### Auth Options (v5.0 only)

| Option | Type | Description |
|--------|------|-------------|
| `reasonCode` | number | Auth reason code |
| `authenticationMethod` | string | Authentication method name |
| `authenticationData` | array | Authentication data (byte array) |
| `reasonString` | string | Human-readable reason |
| `userProperties` | array | User properties `[{key, value}, ...]` |

---

### Receive Messages

The `client.recv()` API works identically on both browser and Node.js:

```javascript
while (true) {
    const packet = await client.recv();

    switch (packet.packetType()) {
        case WasmPacketType.Connack: {
            const connack = client.asConnack(packet);
            console.log('Connected:', connack.sessionPresent);
            break;
        }
        case WasmPacketType.Publish: {
            const pub = client.asPublish(packet);
            console.log(`Topic: ${pub.topicName}, Payload: ${pub.payload}`);
            break;
        }
        case WasmPacketType.Suback: {
            const suback = client.asSuback(packet);
            console.log(`SUBACK: packetId=${suback.packetId}`);
            break;
        }
        // ... handle other packet types
    }
}
```

---

## Received Packet Fields Reference

When receiving packets via `client.recv()`, you can access fields using the `client.asXxx(packet)` methods. The following sections document all available fields for each packet type.

### Received CONNACK

```javascript
const connack = client.asConnack(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `sessionPresent` | boolean | Whether a session from a previous connection exists |
| `returnCode` (v3.1.1) | number | Connect return code |
| `reasonCode` (v5.0) | number | Connect reason code |
| `isSuccess()` | boolean | Returns true if connection was accepted |

#### CONNACK Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `sessionExpiryInterval` | number? | Session expiry interval from server |
| `receiveMaximum` | number? | Maximum concurrent QoS 1/2 receives |
| `maximumQos` | number? | Maximum QoS level supported by server |
| `retainAvailable` | boolean? | Whether retain is available |
| `maximumPacketSize` | number? | Maximum packet size |
| `assignedClientIdentifier` | string? | Client ID assigned by server |
| `topicAliasMaximum` | number? | Maximum topic aliases |
| `reasonString` | string? | Human-readable reason |
| `wildcardSubscriptionAvailable` | boolean? | Whether wildcard subscriptions are available |
| `subscriptionIdentifiersAvailable` | boolean? | Whether subscription identifiers are available |
| `sharedSubscriptionAvailable` | boolean? | Whether shared subscriptions are available |
| `serverKeepAlive` | number? | Server-specified keep alive |
| `responseInformation` | string? | Response information |
| `serverReference` | string? | Server reference for redirect |
| `authenticationMethod` | string? | Authentication method |
| `authenticationData` | Uint8Array? | Authentication data |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

### Received PUBLISH

```javascript
const pub = client.asPublish(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `topicName` | string | Topic name |
| `payload` | string? | Payload as UTF-8 string (null if not valid UTF-8) |
| `payloadBytes()` | Uint8Array | Payload as byte array |
| `qos` | number | QoS level (0, 1, 2) |
| `retain` | boolean | Retain flag |
| `dup` | boolean | Duplicate flag |
| `packetId` | number? | Packet identifier (for QoS > 0) |

#### PUBLISH Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `payloadFormatIndicator` | number? | 0=binary, 1=UTF-8 |
| `messageExpiryInterval` | number? | Message expiry in seconds |
| `topicAlias` | number? | Topic alias used |
| `responseTopic` | string? | Response topic for request/response |
| `correlationData` | Uint8Array? | Correlation data |
| `contentType` | string? | Content type (MIME type) |
| `subscriptionIdentifiers()` | number[] | Subscription identifiers matching this message |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |
| `topicNameExtracted` | boolean | True if topic name was restored from topic alias mapping |

**Note:** `topicNameExtracted` indicates that the received PUBLISH packet had an empty topic name with a topic alias, and the library automatically restored the topic name from the alias mapping.

---

### Received SUBACK

```javascript
const suback = client.asSuback(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `packetId` | number | Packet identifier |
| `returnCodes()` (v3.1.1) | number[] | Return codes for each subscription |
| `reasonCodes()` (v5.0) | number[] | Reason codes for each subscription |

#### SUBACK Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `reasonString` | string? | Human-readable reason |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

### Received UNSUBACK

```javascript
const unsuback = client.asUnsuback(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `packetId` | number | Packet identifier |
| `reasonCodes()` (v5.0) | number[] | Reason codes for each topic |

#### UNSUBACK Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `reasonString` | string? | Human-readable reason |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

### Received PUBACK/PUBREC/PUBREL/PUBCOMP

```javascript
const puback = client.asPuback(packet);
const pubrec = client.asPubrec(packet);
const pubrel = client.asPubrel(packet);
const pubcomp = client.asPubcomp(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `packetId` | number | Packet identifier |
| `reasonCode` (v5.0) | number | Reason code |

#### QoS Response Properties (v5.0 only)

| Property | Type | Description |
|----------|------|-------------|
| `reasonString` | string? | Human-readable reason |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

### Received DISCONNECT (v5.0 only)

```javascript
const disconnect = client.asDisconnect(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `reasonCode` | number | Disconnect reason code |
| `sessionExpiryInterval` | number? | Session expiry interval |
| `reasonString` | string? | Human-readable reason |
| `serverReference` | string? | Server reference for redirect |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

### Received AUTH (v5.0 only)

```javascript
const auth = client.asAuth(packet);
```

| Field | Type | Description |
|-------|------|-------------|
| `reasonCode` | number | Auth reason code |
| `authenticationMethod` | string? | Authentication method |
| `authenticationData` | Uint8Array? | Authentication data |
| `reasonString` | string? | Human-readable reason |
| `userProperties()` | Array | User properties `[{key, value}, ...]` |

---

## Browser Demo Tools

### 1. Build WASM Package

```bash
wasm-pack build --target web
```

### 2. Start Web Server

```bash
./start_web_server.sh        # Default port 8080
./start_web_server.sh 9000   # Custom port
```

### 3. Open in Browser

Access `http://localhost:8080` to see available tools:

- **Client Tool** (`client.html`): Interactive MQTT client with Connect, Subscribe, Publish UI
- **Sequence Test** (`sequence_test.html`): Automated MQTT packet sequence testing

### Browser Requirements

- Modern browser with WebSocket support
- For `wss://` (secure WebSocket): HTTPS page or localhost
- For `ws://` (plain WebSocket): HTTP page (note: some ports like 10080 are blocked by browsers)

---

## Development

### Build

```bash
# Browser (ES modules)
wasm-pack build --target web

# Node.js (CommonJS)
wasm-pack build --target nodejs --out-dir pkg-nodejs

# Bundlers (npm-style)
wasm-pack build --target bundler
```

### Run Tests

```bash
# Native tests (mock underlying layer)
cargo test --features native

# WASM tests
wasm-pack test --node

# Node.js integration tests
node nodejs/test/integration.test.js

# Full check (fmt, clippy, build, test)
./check.sh
```

---

## License

MIT
