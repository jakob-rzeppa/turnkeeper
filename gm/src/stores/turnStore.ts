import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

const { socket } = useConnection()

export const useTurnStore = defineStore('turn', () => {
    // The store shall only be modified by events from the backend.
    const playerOrder = ref<{ id: string; name: string }[]>([])
    const isInitialized = ref(false)
    const round = ref({
        roundNumber: 0,
        currentPlayerIndex: 0,
    })

    const currentPlayerId = computed(() => {
        return playerOrder.value[round.value.currentPlayerIndex]?.id ?? null
    })

    // Listener for updates from server
    socket.on(
        'game:turn',
        ({
            playerOrder: newPlayerOrder,
            round: newRound,
            isInitialized: newIsInitialized,
        }: {
            playerOrder: { id: string; name: string }[]
            round: { roundNumber: number; currentPlayerIndex: number }
            isInitialized: boolean
        }) => {
            playerOrder.value = newPlayerOrder
            round.value = {
                roundNumber: newRound.roundNumber,
                currentPlayerIndex: newRound.currentPlayerIndex,
            }
            isInitialized.value = newIsInitialized
        },
    )

    function nextTurn() {
        socket.emit('game:turn:next')
    }

    return {
        playerOrder,
        isInitialized,
        round,
        currentPlayerId,
        nextTurn,
    }
})
