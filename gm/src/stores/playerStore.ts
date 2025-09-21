import type { Player } from '@/types/player'
import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
    state: () => ({
        players: [] as Player[],
    }),
    getters: {
        getPlayerById: (state) => {
            return (id: string) => state.players.find((p) => p.id === id)
        },
    },
})
