import useConnection from '@/composables/connection'
import type { Player } from '@/types/player'
import { defineStore } from 'pinia'
import { ref } from 'vue'

const { socket } = useConnection()

export const usePlayerStore = defineStore('player', () => {
    // The store shall only be modified by events from the backend.
    const players = ref<Player[]>([])
    const getPlayerById = (id: string) => {
        return players.value.find((p) => p.id === id)
    }

    socket.on('players', (newPlayers) => {
        players.value = newPlayers
    })

    function createPlayer(newPlayerName: string) {
        if (!newPlayerName.trim()) {
            return
        }
        socket.emit('players:create', { name: newPlayerName.trim() })
    }

    function updatePlayer(playerId: string, playerData: Partial<Player>): void {
        socket.emit('players:update', {
            playerId,
            playerData,
        })
    }

    function deletePlayer(playerId: string): void {
        socket.emit('players:delete', { playerId })
    }

    function removeStatFromPlayer(playerId: string, statName: string): void {
        socket.emit('players:stats:remove', { playerId, statName })
    }

    return {
        players,
        getPlayerById,
        createPlayer,
        updatePlayer,
        deletePlayer,
        removeStatFromPlayer,
    }
})
