import useConnection from '@/composables/connection'
import type { GmToBackendEventPayloads, Player, PlayerStat } from 'shared-types'

export const usePlayerEmitter = () => {
    const connection = useConnection()

    function createPlayer(newPlayerName: string) {
        const trimmedNewPlayerName = newPlayerName.trim()
        if (!trimmedNewPlayerName) return

        const payload: GmToBackendEventPayloads['players:create'] = {
            name: trimmedNewPlayerName,
        }
        connection.socket.emit('players:create', payload)
    }

    function updatePlayer(playerId: string, playerData: Partial<Player>): void {
        const payload: GmToBackendEventPayloads['players:update'] = {
            playerId,
            playerData,
        }
        connection.socket.emit('players:update', payload)
    }

    function deletePlayer(playerId: string): void {
        const payload: GmToBackendEventPayloads['players:delete'] = { playerId }
        connection.socket.emit('players:delete', payload)
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
        connection.socket.emit('players:stats:create', payload)
    }

    function removeStatFromPlayer(playerId: string, statName: string): void {
        const payload: GmToBackendEventPayloads['players:stats:remove'] = { playerId, statName }
        connection.socket.emit('players:stats:remove', payload)
    }

    return {
        createPlayer,
        updatePlayer,
        deletePlayer,
        createStatForPlayer,
        removeStatFromPlayer,
    }
}
