import { computed, ref } from 'vue';
import { useGameStore, type Stat, type Player, type Tradable } from '../game/gameStore';
import { useAuthStore } from '../auth/authStore';
import axios from 'axios';
import { API_BASE_URL } from './httpApi';

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

const websocket = ref<WebSocket | null>(null);

// URL management helpers
const saveGameIdToUrl = (gameId: string) => {
    const url = new URL(window.location.href);
    url.searchParams.set('gameId', gameId);
    window.history.replaceState({}, '', url);
};

const removeGameIdFromUrl = () => {
    const url = new URL(window.location.href);
    url.searchParams.delete('gameId');
    window.history.replaceState({}, '', url);
};

const getGameIdFromUrl = (): string | null => {
    const url = new URL(window.location.href);
    return url.searchParams.get('gameId');
};

export function useWsConnection() {
    const gameStore = useGameStore();
    const authStore = useAuthStore();

    const connect = async (gameId: string) => {
        if (websocket.value) {
            console.warn('WebSocket is already connected.');
            return;
        }

        // Fetch a short-lived ticket URL from the authenticated HTTP endpoint
        let wsTicket: string;
        try {
            const response = await axios.post<{ ticket: string }>(
                `${API_BASE_URL}/game/ws/ticket`,
                null,
                { headers: { Authorization: `Bearer ${authStore.token}` } }
            );
            wsTicket = response.data.ticket;
        } catch (err) {
            console.error('Failed to obtain WebSocket ticket:', err);
            removeGameIdFromUrl();
            return;
        }

        console.log('Connecting to WebSocket at:', wsTicket);
        websocket.value = new WebSocket(`${API_BASE_URL}/game/ws/${gameId}?ticket=${wsTicket}`);

        websocket.value.onopen = () => {
            console.log('WebSocket connection established.');
            saveGameIdToUrl(gameId);

            // Send an initial message to trigger the server to send the current game state
            send(JSON.stringify('Connect'));
        };

        websocket.value.onmessage = event => {
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

        websocket.value.onclose = () => {
            console.log('WebSocket connection closed.');
            websocket.value = null;
            removeGameIdFromUrl();
        };

        websocket.value.onerror = error => {
            console.error('WebSocket error:', error);
            removeGameIdFromUrl();
        };
    };

    const send = (message: string) => {
        if (websocket.value && websocket.value.readyState === WebSocket.OPEN) {
            websocket.value.send(message);
        } else {
            console.warn('WebSocket is not connected. Cannot send message.');
        }
    };

    const isConnected = computed(() => websocket.value !== null);

    const disconnect = () => {
        if (websocket.value) {
            websocket.value.close();
            websocket.value = null;
            removeGameIdFromUrl();
        } else {
            console.warn('WebSocket is not connected.');
        }
    };

    const autoConnect = async () => {
        const gameId = getGameIdFromUrl();
        if (gameId) {
            await connect(gameId);
        }
    };

    return { connect, disconnect, isConnected, send, autoConnect, getGameIdFromUrl };
}
