import { computed, ref } from 'vue';
import { API_BASE_URL, postWithAuth } from '../api/httpApi';
import { isDisplayTemplate } from './types/displayTemplate.guard';
import { isGameState } from './types/state.guard';
import type { DisplayTemplate } from './types/displayTemplate';
import type { GameState } from './types/state';

const connection = ref<
        { status: 'disconnected' } | 
        { status: 'connecting' } | 
        { status: 'connected', websocket: WebSocket, displayTemplate: DisplayTemplate | null, state: GameState | null } |
        { status: 'error', error: string }
    >({ status: 'disconnected' });

export function useSessionConnection() {
    const connectionStatus = computed(() => connection.value.status);
    const displayTemplate = computed(() => {
        if (connection.value.status === 'connected') {
            return connection.value.displayTemplate;
        }
        return null;
    });
    const gameState = computed(() => {
        if (connection.value.status === 'connected') {
            return connection.value.state;
        }
        return null;
    });

    const connect = async (gameId: string, gameInstanceId: string) => {
        if (connection.value.status === 'connected') {
            console.warn('WebSocket is already connected.');
            return;
        }

        if (connection.value.status === 'connecting') {
            console.warn('WebSocket is already connecting.');
            return;
        }

        // Set status to connecting before attempting to connect
        connection.value = { status: 'connecting' };

        // Fetch a short-lived ticket URL from the authenticated HTTP endpoint
        const response = await postWithAuth<{ ticket: string }>(
            `/ws/ticket`,
            null
        );
        
        if (response.isErr()) {
            console.error('Failed to obtain WebSocket ticket:', response.error);
            return;
        } 
        let wsTicket = response.value.data.ticket;

        console.log('Connecting to Game Session...');
        const websocket = new WebSocket(`${API_BASE_URL}/games/${gameId}/instances/${gameInstanceId}/ws?ticket=${wsTicket}`);

        websocket.onopen = () => {
            console.log('WebSocket connection established.');
            connection.value = { 
                status: 'connected', 
                websocket,
                displayTemplate: null,
                state: null
            };

            // Send an initial message to trigger the server to send the current game state
            send(JSON.stringify('Connect'));
        };

        websocket.onmessage = event => {
            if (connection.value.status !== 'connected') {
                console.warn('Received message while WebSocket is not connected. This should not happen. Ignoring message.');
                return;
            }

            if (event.data.startsWith('DisplayTemplate ')) {
                const message = JSON.parse(event.data.slice(16));
                if (isDisplayTemplate(message)) {
                    console.log('Received DisplayTemplate:', message);
                    connection.value.displayTemplate = message;
                } else {
                    console.warn('Received invalid DisplayTemplate message:', message);
                }
            } else if (event.data.startsWith('State ')) {
                const message = JSON.parse(event.data.slice(6));
                if (isGameState(message)) {
                    console.log('Received GameState:', message);
                    connection.value.state = message;
                } else {
                    console.warn('Received invalid GameState message:', message);
                }
            } else {
                console.log('Received unrecognized message:', event.data);
            }
        };

        websocket.onclose = () => {
            console.log('Disconnected from Game');
            connection.value = { status: 'disconnected' };
        };

        websocket.onerror = _ => {
            connection.value = { status: 'error', error: 'Connecting to Game Session failed.' };
        };
    };

    const send = (message: string) => {
        if (connection.value.status !== 'connected') {
            console.warn('WebSocket is not connected. Cannot send message.');
            return;
        }

        if (connection.value.websocket && connection.value.websocket.readyState === WebSocket.OPEN) {
            connection.value.websocket.send(message);
        } else {
            console.warn('WebSocket is not connected. Cannot send message.');
        }
    };

    const disconnect = () => {
        if (connection.value.status !== 'connected') {
            console.warn('WebSocket is not connected. Cannot disconnect.');
            return;
        }

        if (connection.value.websocket) {
            console.log('Disconnecting from Game Session...');
            connection.value.websocket.close();
            console.log('Game Session disconnected.');
            connection.value = { status: 'disconnected' };
        } else {
            console.warn('WebSocket is not connected.');
        }
    };

    return { connect, disconnect, send, connectionStatus, displayTemplate, gameState };
}
