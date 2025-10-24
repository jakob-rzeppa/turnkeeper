import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import type { BackendToUserPayloads } from 'shared-types'
import { computed, ref } from 'vue'

export const useGameStore = defineStore('game', () => {
    const playerOrder = ref<{ id: number; name: string }[]>([])
    const round = ref<{ currentPlayerIndex: number; roundNumber: number }>({
        currentPlayerIndex: 0,
        roundNumber: 0,
    })
    const isInitialized = ref<boolean>(false)

    const currentPlayer = computed<{ id: number; name: string } | null>(
        () => playerOrder.value[round.value.currentPlayerIndex] ?? null,
    )

    const connection = useConnection()

    connection.socket.on('game:info', ({ gameState }: BackendToUserPayloads['game:info']) => {
        if (!gameState) {
            isInitialized.value = false
            return
        }

        round.value = {
            currentPlayerIndex: gameState.currentPlayerIndex,
            roundNumber: gameState.roundNumber,
        }
        isInitialized.value = true
        playerOrder.value = gameState.playerOrder
    })

    return { round, isInitialized, playerOrder, currentPlayer }
})
