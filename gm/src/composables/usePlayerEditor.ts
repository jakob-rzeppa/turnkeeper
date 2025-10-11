import { usePlayerStore } from '@/stores/playerStore'
import { reactive, watch } from 'vue'

export const usePlayerEditor = (playerId: string, closeFunction?: () => void) => {
    const playerStore = usePlayerStore()

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

    function updatePlayer(): void {
        playerStore.updatePlayer(playerId, {
            name: localPlayer.name,
            secret: localPlayer.secret,
            stats: localPlayer.stats,
        })
        if (closeFunction) closeFunction()
    }

    return {
        localPlayer,
        updatePlayer,
    }
}
