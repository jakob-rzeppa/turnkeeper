import io from 'socket.io-client';
import { getCookie, removeCookie, setCookie } from 'typescript-cookie';
import { ref } from 'vue';

const socket = io(import.meta.env.VITE_BACKEND_URL, {
    autoConnect: false,
});
const isConnected = ref(socket.connected);
let connectionTimeoutId: number | null = null;

const CONNECTION_TIMEOUT = 2000; // 2 seconds

socket.on('connect', () => {
    console.log('Received connect event from server');

    // Clear the timeout since we received the success confirmation
    if (connectionTimeoutId) {
        clearTimeout(connectionTimeoutId);
        connectionTimeoutId = null;
    }
    isConnected.value = true;
    setCookie('auth', JSON.stringify(socket.auth), { expires: 7 });
});

socket.on('disconnect', () => {
    // Clear any pending timeout
    if (connectionTimeoutId) {
        clearTimeout(connectionTimeoutId);
        connectionTimeoutId = null;
    }
    isConnected.value = false;
    removeCookie('auth');
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
    function connect({ playerName, playerSecret }: { playerName: string; playerSecret: string }) {
        if (!socket.connected) {
            socket.auth = { playerName, playerSecret };
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

    function connectWithCookie() {
        const authCookie = getCookie('auth');
        if (authCookie) {
            try {
                const auth = JSON.parse(authCookie);
                connect({ playerName: auth.playerName, playerSecret: auth.playerSecret });
            } catch (error) {
                console.error('Failed to parse auth cookie:', error);
            }
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
        connectWithCookie,
        disconnect,
    };
}
