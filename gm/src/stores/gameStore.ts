import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToGmEventPayloads, GameState } from 'shared-types';
import { computed, ref } from 'vue';

const { socket } = useConnection();

// The game store stores all the information about the current game state.
export const useGameStore = defineStore('game', () => {
    // The store shall only be modified by events from the backend.
    const playerOrder = ref<GameState['playerOrder']>([]);
    const isInitialized = ref<boolean>(false);
    const round = ref<Pick<GameState, 'roundNumber' | 'currentPlayerIndex'>>({
        roundNumber: 0,
        currentPlayerIndex: 0,
    });

    const currentPlayerId = computed(() => {
        return playerOrder.value[round.value.currentPlayerIndex]?.id ?? null;
    });

    // Listener for updates from server
    socket.on('game:info', ({ gameState }: BackendToGmEventPayloads['game:info']) => {
        if (!gameState) {
            isInitialized.value = false;
            return;
        }

        round.value = {
            currentPlayerIndex: gameState.currentPlayerIndex,
            roundNumber: gameState.roundNumber,
        };
        isInitialized.value = true;
        playerOrder.value = gameState.playerOrder;
    });

    return {
        playerOrder,
        isInitialized,
        round,
        currentPlayerId,
    };
});
