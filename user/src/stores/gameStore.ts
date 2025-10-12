import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useGameStore = defineStore('game', () => {
    const playerOrder = ref<{ id: string; name: string }[]>([])
    const round = ref<{ currentPlayerIndex: number; roundNumber: number }>({
        currentPlayerIndex: 0,
        roundNumber: 0,
    })
    const isInitialized = ref<boolean>(false)

    const currentPlayer = computed<{ id: string; name: string } | null>(
        () => playerOrder.value[round.value.currentPlayerIndex] ?? null,
    )

    const connection = useConnection()

    connection.socket.on(
        'game',
        ({
            round: newRound,
            isInitialized: newIsInitialized,
            playerOrder: newPlayerOrder,
        }: {
            round: { currentPlayerIndex: number; roundNumber: number }
            isInitialized: boolean
            playerOrder: { id: string; name: string }[]
        }) => {
            round.value = newRound
            isInitialized.value = newIsInitialized
            playerOrder.value = newPlayerOrder
        },
    )

    return { round, isInitialized, playerOrder, currentPlayer }
})
