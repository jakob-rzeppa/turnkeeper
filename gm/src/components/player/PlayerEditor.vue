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
    <h1 class="text-4xl text-center text-primary">Edit player</h1>
    <label class="input input-primary w-full">
        <span class="label">Name</span>
        <input type="text" v-model="localPlayer.name" />
    </label>
    <label class="input input-primary w-full">
        <span class="label">Secret</span>
        <input type="text" v-model="localPlayer.secret" />
    </label>
    <PlayerStatsEditor
        :player-id="props.playerId"
        :player-name="localPlayer.name"
        :player-stats="localPlayer.stats"
    />
    <button class="btn btn-primary btn-lg" @click="updatePlayer">Update Player</button>
</template>
