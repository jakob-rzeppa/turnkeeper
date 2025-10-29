import io from 'socket.io-client';
import { ref } from 'vue';

const socket = io('http://localhost:3000/gm', { autoConnect: false });
const isConnected = ref(socket.connected);
let connectionTimeoutId: NodeJS.Timeout | null = null;

const CONNECTION_TIMEOUT = 2000; // 2 seconds

socket.on('connect', () => {
    console.log('Received connect event from server');

    // Clear the timeout since we received the success confirmation
    if (connectionTimeoutId) {
        clearTimeout(connectionTimeoutId);
        connectionTimeoutId = null;
    }
    isConnected.value = true;
});

socket.on('disconnect', () => {
    // Clear any pending timeout
    if (connectionTimeoutId) {
        clearTimeout(connectionTimeoutId);
        connectionTimeoutId = null;
    }
    isConnected.value = false;
});

socket.on('connection_error', (error) => {
    console.error('Connection refused:', error.message);
    alert(`Connection refused: ${error.message}`);

    // Clear any pending timeout
    if (connectionTimeoutId) {
        clearTimeout(connectionTimeoutId);
        connectionTimeoutId = null;
    }

    isConnected.value = false;
    socket.disconnect();
});

socket.onAnyOutgoing((event, ...args) => {
    console.log(`Socket event emitted: ${event}`, args);
});

socket.onAny((event, ...args) => {
    console.log(`Socket event received: ${event}`, args);
});

export default function useConnection() {
    function connect() {
        if (!socket.connected) {
            socket.connect();

            // Start timeout - if we don't receive connect event, disconnect
            console.log('Socket connected, waiting for connect event...');
            connectionTimeoutId = setTimeout(() => {
                console.error('Connection timeout: Did not receive connect event');
                alert('Connection timeout: Did not receive connect event');
                socket.disconnect();
            }, CONNECTION_TIMEOUT);
        }
    }

    function disconnect() {
        if (socket.connected) {
            socket.disconnect();
        }
    }

    return {
        socket,
        isConnected,
        connect,
        disconnect,
    };
}
