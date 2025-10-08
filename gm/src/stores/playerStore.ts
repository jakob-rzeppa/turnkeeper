import useConnection from '@/composables/connection'
import type { Player } from '@/types/player'
import { defineStore } from 'pinia'
import { ref } from 'vue'

const { socket } = useConnection()

export const usePlayerStore = defineStore('player', () => {
    const players = ref<Player[]>([])
    const getPlayerById = (id: string) => {
        return players.value.find((p) => p.id === id)
    }

    socket.on('players', (newPlayers) => {
        players.value = newPlayers
    })

    return {
        players,
        getPlayerById,
    }
})
