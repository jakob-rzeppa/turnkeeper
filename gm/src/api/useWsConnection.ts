import { computed, ref } from 'vue';
import { useGameStore, type Stat, type Player } from '../game/gameStore';
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
type RawPlayer = { id: string; user_id: string | null; stats: RawStat[] };
type RawGame = {
    id: string;
    name: string;
    players: RawPlayer[];
    round_number: number;
    current_player_index: number;
    notes: string;
    hidden_notes: string;
};

const websocket = ref<WebSocket | null>(null);

export function useWsConnection() {
    const gameStore = useGameStore();
    const authStore = useAuthStore();

    const connect = async (gameId: string) => {
        if (websocket.value) {
            console.warn('WebSocket is already connected.');
            return;
        }

        // Fetch a short-lived ticket URL from the authenticated HTTP endpoint
        let wsUrl: string;
        try {
            const response = await axios.post<{ url: string }>(
                `${API_BASE_URL}/ws/ticket/${gameId}`,
                null,
                { headers: { Authorization: `Bearer ${authStore.token}` } }
            );
            wsUrl = response.data.url;
        } catch (err) {
            console.error('Failed to obtain WebSocket ticket:', err);
            return;
        }

        console.log('Connecting to WebSocket at:', wsUrl);
        websocket.value = new WebSocket(wsUrl);

        websocket.value.onopen = () => {
            console.log('WebSocket connection established.');
        };

        websocket.value.onmessage = event => {
            console.log('Received message:', event.data);

            const message = JSON.parse(event.data) as RawGame;

            gameStore.setGame({
                id: message.id,
                name: message.name,
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
        };

        websocket.value.onerror = error => {
            console.error('WebSocket error:', error);
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
        } else {
            console.warn('WebSocket is not connected.');
        }
    };

    return { connect, disconnect, isConnected, send };
}
