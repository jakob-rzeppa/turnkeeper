import { defineStore } from 'pinia'

export const useTurnStore = defineStore('turn', {
    state: () => ({
        playerOrder: [] as { id: string; name: string }[],
        isInitialized: false,
        round: {
            roundNumber: 0,
            currentPlayerIndex: 0,
        },
    }),
    getters: {
        currentPlayerId: (state) => {
            return state.playerOrder[state.round.currentPlayerIndex]?.id ?? null
        },
    },
})
