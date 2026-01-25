/**
 * MQTT Broker helper for integration tests
 *
 * Manages starting/stopping mqtt-broker-tokio as a child process
 */

const { spawn, execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

// Paths
const PROJECT_ROOT = path.join(__dirname, '..', '..');
const WORK_DIR = path.join(PROJECT_ROOT, '.work');
const BROKER_DIR = path.join(WORK_DIR, 'mqtt-broker-tokio');
const BROKER_PATH = path.join(BROKER_DIR, 'target', 'release', 'mqtt-broker');
const CERTS_DIR = path.join(PROJECT_ROOT, 'tests', 'certs');

const PORTS = {
    TCP: 21883,
    TLS: 28883,
    WS: 20080,
    WSS: 20443
};

/**
 * Ensure the broker is cloned and built
 */
function ensureBroker() {
    // Create work directory if needed
    if (!fs.existsSync(WORK_DIR)) {
        fs.mkdirSync(WORK_DIR, { recursive: true });
    }

    // Clone if not present
    if (!fs.existsSync(BROKER_DIR)) {
        console.log('Cloning mqtt-broker-tokio...');
        execSync('git clone --depth 1 https://github.com/redboltz/mqtt-broker-tokio', {
            cwd: WORK_DIR,
            stdio: 'inherit'
        });
    }

    // Build if not present
    if (!fs.existsSync(BROKER_PATH)) {
        console.log('Building mqtt-broker-tokio...');
        execSync('cargo build --release', {
            cwd: BROKER_DIR,
            stdio: 'inherit'
        });
    }

    return BROKER_PATH;
}

/**
 * Start the MQTT broker as a child process
 * @returns {Promise<ChildProcess>} The broker process
 */
function startBroker() {
    return new Promise((resolve, reject) => {
        // Ensure broker exists
        let brokerPath;
        try {
            brokerPath = ensureBroker();
        } catch (e) {
            reject(new Error(`Failed to setup broker: ${e.message}`));
            return;
        }

        const args = [
            '--tcp-port', PORTS.TCP.toString(),
            '--tls-port', PORTS.TLS.toString(),
            '--ws-port', PORTS.WS.toString(),
            '--ws-tls-port', PORTS.WSS.toString(),
            '--server-crt', path.join(CERTS_DIR, 'server.crt.pem'),
            '--server-key', path.join(CERTS_DIR, 'server.key.pem')
        ];

        const broker = spawn(brokerPath, args, {
            stdio: ['ignore', 'pipe', 'pipe']
        });

        let started = false;
        let output = '';

        const checkStarted = (data) => {
            output += data.toString();
            // Check for broker startup indicators
            if (!started && (output.includes('Listening') || output.includes('Started') || output.includes('TCP') || output.includes('listening'))) {
                started = true;
                // Give a bit more time for all listeners to be ready
                setTimeout(() => resolve(broker), 100);
            }
        };

        broker.stdout.on('data', checkStarted);
        broker.stderr.on('data', checkStarted);

        broker.on('error', (err) => {
            if (!started) {
                reject(new Error(`Failed to start broker: ${err.message}`));
            }
        });

        broker.on('exit', (code) => {
            if (!started) {
                reject(new Error(`Broker exited with code ${code} before starting. Output: ${output}`));
            }
        });

        // Timeout for startup
        setTimeout(() => {
            if (!started) {
                broker.kill();
                // If no explicit startup message, assume it's ready after timeout
                started = true;
                resolve(broker);
            }
        }, 2000);
    });
}

/**
 * Stop the MQTT broker
 * @param {ChildProcess} broker - The broker process to stop
 * @returns {Promise<void>}
 */
function stopBroker(broker) {
    return new Promise((resolve) => {
        if (!broker || broker.killed) {
            resolve();
            return;
        }

        broker.on('exit', () => {
            resolve();
        });

        broker.kill('SIGTERM');

        // Force kill after timeout
        setTimeout(() => {
            if (!broker.killed) {
                broker.kill('SIGKILL');
            }
            resolve();
        }, 2000);
    });
}

/**
 * Get connection URLs for different transports
 */
function getConnectionInfo() {
    return {
        tcp: { host: 'localhost', port: PORTS.TCP },
        tls: { host: 'localhost', port: PORTS.TLS },
        ws: `ws://localhost:${PORTS.WS}/`,
        wss: `wss://localhost:${PORTS.WSS}/`,
        certs: {
            ca: path.join(CERTS_DIR, 'cacert.pem'),
            cert: path.join(CERTS_DIR, 'server.crt.pem'),
            key: path.join(CERTS_DIR, 'server.key.pem')
        }
    };
}

module.exports = {
    startBroker,
    stopBroker,
    getConnectionInfo,
    ensureBroker,
    PORTS,
    BROKER_PATH,
    BROKER_DIR
};
