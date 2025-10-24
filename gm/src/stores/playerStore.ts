import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { BackendToGmEventPayloads, Player } from 'shared-types';

const { socket } = useConnection();

export const usePlayerStore = defineStore('player', () => {
    // The store shall only be modified by events from the backend.
    const players = ref<Player[]>([]);
    const getPlayerById = (id: number) => {
        return players.value.find((p) => p.id === id);
    };

    socket.on(
        'players:info',
        ({ players: newPlayers }: BackendToGmEventPayloads['players:info']) => {
            players.value = newPlayers;
        },
    );
    return {
        players,
        getPlayerById,
    };
});
