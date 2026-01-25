# mqtt-client-wasm

[![CI](https://github.com/redboltz/mqtt-client-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/redboltz/mqtt-client-wasm/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/redboltz/mqtt-client-wasm/branch/main/graph/badge.svg)](https://codecov.io/gh/redboltz/mqtt-client-wasm)

MQTT client library for browsers using WebSocket (ws/wss) transport, compiled to WebAssembly.

This library provides a WebSocket-based MQTT client specifically designed for browser environments. It supports both MQTT v3.1.1 and v5.0 protocols over WebSocket connections (ws:// and wss://), making it ideal for web applications that need to communicate with MQTT brokers.

## Features

- **Browser WebSocket Transport**: Uses browser's native WebSocket API for ws:// and wss:// connections
- **MQTT v3.1.1 and v5.0**: Full support for both protocol versions
- **Low-level Endpoint API**: Direct packet send/recv operations for full control
- **Auto Response Options**: Configurable automatic handling of PUBACK, PUBREC, PUBREL, PUBCOMP, and PINGRESP
- **Interactive Client Tool**: Ready-to-use HTML client for testing and debugging
- **Sequence Test Tool**: Automated MQTT packet sequence testing

## Installation

### npm (Recommended for bundler projects)

```bash
npm install @redboltz/mqtt-client-wasm
```

Then import in your JavaScript/TypeScript:

```javascript
import init, { WasmMqttClient, WasmMqttConfig, WasmPacketType } from '@redboltz/mqtt-client-wasm';

async function main() {
    await init();
    // ... use the client
}
```

Works with webpack, vite, rollup, and other bundlers.

### Direct Download (No bundler)

Download the latest release from [GitHub Releases](https://github.com/redboltz/mqtt-client-wasm/releases) and include in your HTML:

```html
<script type="module">
    import init, { WasmMqttClient, WasmMqttConfig } from './pkg/mqtt_client_wasm.js';
    await init();
    // ... use the client
</script>
```

### Build from Source

```bash
# For direct browser use (ES modules)
wasm-pack build --target web

# For bundlers (npm-style)
wasm-pack build --target bundler
```

## Quick Start (Demo)

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

## JavaScript API

### Configuration

```javascript
import init, { WasmMqttClient, WasmMqttConfig } from './pkg/mqtt_client_wasm.js';

await init();

const config = new WasmMqttConfig({
    version: '5.0',
    pingreqSendIntervalMs: 30000,
    autoPubResponse: true,
    autoPingResponse: true,
});

const client = new WasmMqttClient(config);
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `'3.1.1'` | MQTT version (`'3.1.1'` or `'5.0'`) |
| `pingreqSendIntervalMs` | number | null | Ping interval in ms (null = auto from keepAlive) |
| `autoPubResponse` | boolean | false | Auto handle QoS acknowledgments |
| `autoPingResponse` | boolean | false | Auto respond to PINGREQ |
| `autoMapTopicAliasSend` | boolean | false | Auto map topic aliases (v5.0) |
| `autoReplaceTopicAliasSend` | boolean | false | Auto replace topic with alias (v5.0) |
| `pingrespRecvTimeoutMs` | number | null | PINGRESP timeout in ms |
| `connectionEstablishTimeoutMs` | number | null | Connection timeout in ms |
| `shutdownTimeoutMs` | number | null | Shutdown timeout in ms |

---

## Packet Reference

### Connect

```javascript
await client.connect('wss://broker.example.com:8884/');

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

```javascript
while (true) {
    const packet = await client.recv();

    switch (packet.packetType()) {
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

## Browser Requirements

- Modern browser with WebSocket support
- For `wss://` (secure WebSocket): HTTPS page or localhost
- For `ws://` (plain WebSocket): HTTP page (note: some ports like 10080 are blocked by browsers)

## Development

### Run Tests

```bash
# Native tests (mock WebSocket)
cargo test --features native

# WASM tests
wasm-pack test --node

# Full check (fmt, clippy, build, test)
./check.sh
```

### Build

```bash
# Debug build
wasm-pack build --target web --dev

# Release build
wasm-pack build --target web
```

## License

MIT
