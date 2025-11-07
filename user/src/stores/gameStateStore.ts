import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import { type BackendToUserEventPayloads, type UserGameState } from 'shared-types';
import { computed, ref } from 'vue';

export const useGameStateStore = defineStore('game', () => {
    const gameState = ref<UserGameState | null>(null);

    const currentPlayer = computed<{ id: number; name: string } | null>(
        () => gameState.value?.playerOrder[gameState.value.currentPlayerIndex] ?? null,
    );

    const connection = useConnection();

    connection.socket.on(
        'game:info',
        ({ gameState: newGameState }: BackendToUserEventPayloads['game:info']) => {
            gameState.value = newGameState;
        },
    );

    return { gameState, currentPlayer };
});
