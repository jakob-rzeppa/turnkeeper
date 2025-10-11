import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

const { socket } = useConnection()

// The game store stores all the information about the current game state.
export const useGameStore = defineStore('game', () => {
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
        'game',
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

    function initGame(playerIdsInOrder: string[]) {
        socket.emit('game:init', { playerIdsInOrder })
    }

    function endGame() {
        socket.emit('game:end')
    }

    function updatePlayerOrder(playerIdsInOrder: string[]) {
        socket.emit('game:playerOrder:update', {
            playerIdsInOrder,
        })
    }

    function nextTurn() {
        socket.emit('game:turn:next')
    }

    return {
        playerOrder,
        isInitialized,
        round,
        currentPlayerId,
        nextTurn,
        initGame,
        updatePlayerOrder,
        endGame,
    }
})
