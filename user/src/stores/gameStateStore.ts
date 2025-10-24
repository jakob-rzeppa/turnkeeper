import useConnection from '@/composables/connection';
import { defineStore } from 'pinia';
import { type GameState, type BackendToUserPayloads } from 'shared-types';
import { computed, ref } from 'vue';

export const useGameStateStore = defineStore('game', () => {
    const gameState = ref<GameState | null>(null);

    const currentPlayer = computed<{ id: number; name: string } | null>(
        () => gameState.value?.playerOrder[gameState.value.currentPlayerIndex] ?? null,
    );

    const connection = useConnection();

    connection.socket.on(
        'game:info',
        ({ gameState: newGameState }: BackendToUserPayloads['game:info']) => {
            gameState.value = newGameState;
        },
    );

    return { gameState, currentPlayer };
});
