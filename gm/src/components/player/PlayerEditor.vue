<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import { reactive, watch } from 'vue'
import PlayerStatsEditor from './PlayerStatsEditor.vue'

const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['done'])

const playerStore = usePlayerStore()

const player = playerStore.getPlayerById(props.playerId) ?? {
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
    () => playerStore.getPlayerById(props.playerId),
    (updatedPlayer) => {
        // When the player is not found (deleted), close the modal
        if (!updatedPlayer) {
            emit('done')
            return
        }

        localPlayer.name = updatedPlayer.name
        localPlayer.secret = updatedPlayer.secret
        localPlayer.stats = updatedPlayer.stats.map((s) => ({ ...s }))
    },
)

function updatePlayer(): void {
    playerStore.updatePlayer(props.playerId, {
        name: localPlayer.name,
        secret: localPlayer.secret,
        stats: localPlayer.stats,
    })
    emit('done')
}
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Edit Player</h1>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Player Name</span>
                </label>
                <input
                    type="text"
                    v-model="localPlayer.name"
                    class="input input-bordered input-primary w-full"
                    placeholder="Enter player name..."
                />
            </div>

            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Secret Code</span>
                </label>
                <input
                    type="text"
                    v-model="localPlayer.secret"
                    class="input input-bordered input-secondary w-full"
                    placeholder="Enter secret code..."
                />
            </div>
        </div>

        <PlayerStatsEditor
            :player-id="props.playerId"
            :player-name="localPlayer.name"
            :player-stats="localPlayer.stats"
        />

        <div class="modal-action">
            <button class="btn btn-primary btn-lg w-full" @click="updatePlayer">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M5 13l4 4L19 7"
                    ></path>
                </svg>
                Update Player
            </button>
        </div>
    </div>
</template>
