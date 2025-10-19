import useConnection from '@/composables/useConnection'
import { defineStore } from 'pinia'
import type { BackendToGmEventPayloads, GameState } from 'shared-types'
import { computed, ref } from 'vue'

const { socket } = useConnection()

// The game store stores all the information about the current game state.
export const useGameStore = defineStore('game', () => {
    // The store shall only be modified by events from the backend.
    const playerOrder = ref<GameState['playerOrder']>([])
    const isInitialized = ref<GameState['isInitialized']>(false)
    const round = ref<GameState['round']>({
        roundNumber: 0,
        currentPlayerIndex: 0,
    })

    const currentPlayerId = computed(() => {
        return playerOrder.value[round.value.currentPlayerIndex]?.id ?? null
    })

    // Listener for updates from server
    socket.on(
        'game:info',
        ({
            playerOrder: newPlayerOrder,
            round: newRound,
            isInitialized: newIsInitialized,
        }: BackendToGmEventPayloads['game:info']) => {
            playerOrder.value = newPlayerOrder
            round.value = {
                roundNumber: newRound.roundNumber,
                currentPlayerIndex: newRound.currentPlayerIndex,
            }
            isInitialized.value = newIsInitialized
        },
    )

    return {
        playerOrder,
        isInitialized,
        round,
        currentPlayerId,
    }
})
