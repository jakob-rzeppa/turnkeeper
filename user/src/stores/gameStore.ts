import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useGameStore = defineStore('game', () => {
    const round = ref<{ currentPlayerIndex: number; roundNumber: number }>({
        currentPlayerIndex: 0,
        roundNumber: 0,
    })
    const isInitialized = ref<boolean>(false)

    const connection = useConnection()

    connection.socket.on(
        'game',
        ({
            round: newRound,
            isInitialized: newIsInitialized,
        }: {
            round: { currentPlayerIndex: number; roundNumber: number }
            isInitialized: boolean
        }) => {
            round.value = newRound
            isInitialized.value = newIsInitialized
        },
    )

    return { round, isInitialized }
})
