import useConnection from '@/composables/connection'
import type { GmToBackendEventPayloads } from 'shared-types'

export const useGameEmitter = () => {
    const connection = useConnection()

    function initGame(playerIdsInOrder: number[]) {
        const payload: GmToBackendEventPayloads['game:init'] = {
            playerIdsInOrder,
        }

        connection.socket.emit('game:init', payload)
    }

    function endGame() {
        connection.socket.emit('game:end')
    }

    function updatePlayerOrder(playerIdsInOrder: number[]) {
        const payload: GmToBackendEventPayloads['game:playerOrder:update'] = {
            playerIdsInOrder,
        }

        connection.socket.emit('game:playerOrder:update', payload)
    }

    function nextTurn() {
        connection.socket.emit('game:turn:next')
    }

    return {
        initGame,
        endGame,
        updatePlayerOrder,
        nextTurn,
    }
}
