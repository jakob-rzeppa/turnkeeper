import { defineStore } from 'pinia'

type Player = { name: string; stats: { [key: string]: number | boolean | string | string[] }[] }

export const usePlayerStore = defineStore('player', {
    state: () => ({
        players: [] as Player[],
    }),
})
