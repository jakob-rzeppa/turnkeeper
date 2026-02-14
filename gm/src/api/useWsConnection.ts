import { computed, ref } from 'vue';

const WS_BASE_URL = 'ws://localhost:8080/gm/ws';

const websocket = ref<WebSocket | null>(null);

export function useWsConnection() {
    const connect = (gameId: string) => {
        if (websocket.value) {
            console.warn('WebSocket is already connected.');
            return;
        }

        websocket.value = new WebSocket(WS_BASE_URL + '/' + gameId);

        websocket.value.onopen = () => {
            console.log('WebSocket connection established.');
        };

        websocket.value.onmessage = event => {
            console.log('Received message:', event.data);
        };

        websocket.value.onclose = () => {
            console.log('WebSocket connection closed.');
            websocket.value = null;
        };

        websocket.value.onerror = error => {
            console.error('WebSocket error:', error);
        };
    };

    const isConnected = computed(() => websocket.value !== null);

    const disconnect = () => {
        if (websocket.value) {
            websocket.value.close();
            websocket.value = null;
        } else {
            console.warn('WebSocket is not connected.');
        }
    };

    return { connect, disconnect, isConnected };
}
