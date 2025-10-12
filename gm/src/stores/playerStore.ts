import useConnection from '@/composables/connection'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import type {
    BackendToGmEventPayloads,
    GmToBackendEventPayloads,
    Player,
    PlayerStat,
} from 'shared-types'

const { socket } = useConnection()

export const usePlayerStore = defineStore('player', () => {
    // The store shall only be modified by events from the backend.
    const players = ref<Player[]>([])
    const getPlayerById = (id: string) => {
        return players.value.find((p) => p.id === id)
    }

    socket.on(
        'players:info',
        ({ players: newPlayers }: BackendToGmEventPayloads['players:info']) => {
            players.value = newPlayers
        },
    )

    function createPlayer(newPlayerName: string) {
        const trimmedNewPlayerName = newPlayerName.trim()
        if (!trimmedNewPlayerName) return

        const payload: GmToBackendEventPayloads['players:create'] = {
            name: trimmedNewPlayerName,
        }
        socket.emit('players:create', payload)
    }

    function updatePlayer(playerId: string, playerData: Partial<Player>): void {
        const payload: GmToBackendEventPayloads['players:update'] = {
            playerId,
            playerData,
        }
        socket.emit('players:update', payload)
    }

    function deletePlayer(playerId: string): void {
        const payload: GmToBackendEventPayloads['players:delete'] = { playerId }
        socket.emit('players:delete', payload)
    }

    function createStatForPlayer(
        statData: PlayerStat,
        scope: 'global' | 'player',
        playerId?: string,
    ): void {
        const payload: GmToBackendEventPayloads['players:stats:create'] = {
            scope,
            playerId,
            statData,
        }
        socket.emit('players:stats:create', payload)
    }

    function removeStatFromPlayer(playerId: string, statName: string): void {
        const payload: GmToBackendEventPayloads['players:stats:remove'] = { playerId, statName }
        socket.emit('players:stats:remove', payload)
    }

    return {
        players,
        getPlayerById,
        createPlayer,
        updatePlayer,
        deletePlayer,
        createStatForPlayer,
        removeStatFromPlayer,
    }
})
