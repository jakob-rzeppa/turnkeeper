import { usePlayerEmitter } from '@/emitters/playerEmitter'
import { useGameStore } from '@/stores/gameStore'
import { usePlayerStore } from '@/stores/playerStore'
import { reactive, watch } from 'vue'

/**
 * Composable to edit a player. It provides a local copy of the player, which can be modified and then saved back to the store.
 *
 * @param playerId if no playerId is supplied the current player (whose turn is) is used
 * @param closeFunction the close function will be called, when the editor is done (e.g. a modal should be closed)
 */
export const usePlayerEditor = (playerId: number, closeFunction?: () => void) => {
    const playerStore = usePlayerStore()
    const playerEmitter = usePlayerEmitter()

    const player = playerStore.getPlayerById(playerId) ?? {
        name: '',
        secret: '',
        stats: [],
    }

    // The local player needs to be a deep copy, so that changes to stats do not directly modify the store. The store shall only be modified by events from the backend.
    const localPlayer = reactive({
        name: player.name,
        secret: player.secret,
        stats: player.stats.map((s) => ({ ...s })),
    })

    // Update Player info, when the player in the backend changes
    watch(
        () => playerStore.getPlayerById(playerId),
        (updatedPlayer) => {
            // When the player is not found (deleted), close the modal
            if (!updatedPlayer) {
                if (closeFunction) closeFunction()
                return
            }

            localPlayer.name = updatedPlayer.name
            localPlayer.secret = updatedPlayer.secret
            localPlayer.stats = updatedPlayer.stats.map((s) => ({ ...s }))
        },
    )

    const updatePlayer = (): void => {
        playerEmitter.updatePlayer(playerId, {
            name: localPlayer.name,
            secret: localPlayer.secret,
        })
        if (closeFunction) closeFunction()
    }

    const deletePlayer = (): void => {
        if (
            confirm(
                `Are you sure you want to delete player with id ${playerId}? This action cannot be undone.`,
            )
        ) {
            playerEmitter.deletePlayer(playerId)
            if (closeFunction) closeFunction()
        }
    }

    return {
        localPlayer,
        updatePlayer,
        deletePlayer,
    }
}
