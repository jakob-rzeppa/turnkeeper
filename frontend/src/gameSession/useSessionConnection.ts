import { computed, ref } from 'vue';
import { useGameStore, type Stat, type Player, type Tradable } from '../game/gameStore';
import { API_BASE_URL, postWithAuth } from '../api/httpApi';

type RawStat = {
    id: string;
    key: string;
    value_type: Stat['valueType'];
    string_value: string | null;
    number_value: number | null;
    boolean_value: boolean | null;
};
type RawTradable = {
    id: string;
    name: string;
    value: number;
};
type RawPlayer = { id: string; user_id: string | null; stats: RawStat[]; tradables: RawTradable[] };
type RawGame = {
    id: string;
    name: string;
    gm_user_id: string;
    players: RawPlayer[];
    round_number: number;
    current_player_index: number;
    notes: string;
    hidden_notes: string;
};

const connection = ref<
        { status: 'disconnected' } | 
        { status: 'connecting' } | 
        { status: 'connected', websocket: WebSocket } |
        { status: 'error', error: string }
    >({ status: 'disconnected' });

export function useSessionConnection() {
    const gameStore = useGameStore();

    const connectionStatus = computed(() => connection.value.status);

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
                websocket
            };

            // Send an initial message to trigger the server to send the current game state
            send(JSON.stringify('Connect'));
        };

        websocket.onmessage = event => {
            if (!event.data.startsWith('FullGameProjection ')) {
                console.warn('Received unknown message type:', event.data);
                return;
            }

            console.log('Received message:', event.data);

            const message = JSON.parse(event.data.slice(19)) as RawGame;

            gameStore.setGame({
                id: message.id,
                name: message.name,
                gm_user_id: message.gm_user_id,
                players: message.players.map(
                    (p: RawPlayer): Player => ({
                        id: p.id,
                        userId: p.user_id,
                        stats: p.stats.map(
                            (s: RawStat): Stat => ({
                                id: s.id,
                                key: s.key,
                                valueType: s.value_type,
                                stringValue: s.string_value,
                                numberValue: s.number_value,
                                booleanValue: s.boolean_value,
                            })
                        ),
                        tradables: p.tradables.map(
                            (t: RawTradable): Tradable => ({
                                id: t.id,
                                name: t.name,
                                value: t.value,
                            })
                        ),
                    })
                ),
                roundNumber: message.round_number,
                currentPlayerIndex: message.current_player_index,
                notes: message.notes,
                hiddenNotes: message.hidden_notes,
            });
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

    return { connect, disconnect, send, connectionStatus };
}
