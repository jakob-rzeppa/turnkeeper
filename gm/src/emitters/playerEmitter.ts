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

    function updatePlayer(
        playerId: number,
        playerData: Partial<Omit<Player, 'id' | 'stats'>>,
    ): void {
        const payload: GmToBackendEventPayloads['players:update'] = {
            playerId,
            playerData,
        }
        connection.socket.emit('players:update', payload)
    }

    function deletePlayer(playerId: number): void {
        const payload: GmToBackendEventPayloads['players:delete'] = { playerId }
        connection.socket.emit('players:delete', payload)
    }

    function createStatForPlayer(
        statData: Omit<PlayerStat, 'id'>,
        scope: 'global' | 'player',
        playerId?: number,
    ): void {
        const payload: GmToBackendEventPayloads['players:stats:create'] = {
            scope,
            playerId,
            statData,
        }
        connection.socket.emit('players:stats:create', payload)
    }

    function updateStatValueForPlayer(
        playerId: number,
        statId: number,
        value: PlayerStat['value'],
    ): void {
        const payload: GmToBackendEventPayloads['players:stats:update'] = {
            playerId,
            statId,
            value,
        }
        connection.socket.emit('players:stats:update', payload)
    }

    function removeStatFromPlayer(playerId: number, statId: number): void {
        const payload: GmToBackendEventPayloads['players:stats:remove'] = { playerId, statId }
        connection.socket.emit('players:stats:remove', payload)
    }

    return {
        createPlayer,
        updatePlayer,
        deletePlayer,
        createStatForPlayer,
        updateStatValueForPlayer,
        removeStatFromPlayer,
    }
}
