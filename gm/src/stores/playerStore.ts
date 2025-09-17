import type { Player } from '@/types/player'
import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
    state: () => ({
        players: [] as Player[],
    }),
})
