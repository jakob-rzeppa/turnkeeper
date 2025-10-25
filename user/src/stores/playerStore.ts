import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToUserPayloads, Player } from 'shared-types';
import { ref } from 'vue';

export const usePlayerStore = defineStore('player', () => {
    const player = ref<Omit<Player, 'secret'> | null>(null);

    const connection = useConnection();

    connection.socket.on(
        'player:info',
        ({ player: newPlayer }: BackendToUserPayloads['player:info']) => {
            player.value = newPlayer || null;
        },
    );

    return { player };
});
