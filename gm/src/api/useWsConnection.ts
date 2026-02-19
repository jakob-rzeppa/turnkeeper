import { computed, ref } from 'vue';
import { useGameStore, type Stat, type Player } from '../game/gameStore';

type RawStat = {
    id: string;
    key: string;
    value_type: Stat['valueType'];
    string_value: string | null;
    number_value: number | null;
    boolean_value: boolean | null;
};
type RawPlayer = { id: string; name: string; stats: RawStat[] };
type RawGame = {
    id: string;
    name: string;
    players: RawPlayer[];
    round_number: number;
    current_player_index: number;
};

const WS_BASE_URL = 'ws://localhost:8080/gm/ws';

const websocket = ref<WebSocket | null>(null);

export function useWsConnection() {
    const gameStore = useGameStore();

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

            const message = JSON.parse(event.data) as RawGame;

            gameStore.setGame({
                id: message.id,
                name: message.name,
                players: message.players.map(
                    (p: RawPlayer): Player => ({
                        id: p.id,
                        name: p.name,
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
