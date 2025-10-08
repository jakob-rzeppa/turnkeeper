<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import { ref, watch } from 'vue'
import PlayerStatsEditor from './PlayerStatsEditor.vue'

const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['done'])

const playerStore = usePlayerStore()

/*
 * Even though we update the refs if the playerStore changes we get the initial values from the props.
 * That way we don't have to deal with a possibly undefined player while the modal is open.
 */
const playerNameRef = ref('')
const playerSecretRef = ref('')
const playerStatsRef = ref<{ name: string; value: boolean | number | string | string[] }[]>([])

watch(
    () => props.playerId,
    (playerId) => {
        const newPlayer = playerStore.getPlayerById(playerId)
        console.log('Loaded player:', newPlayer)
        if (!newPlayer) return

        playerNameRef.value = newPlayer.name
        playerSecretRef.value = newPlayer.secret
        playerStatsRef.value = newPlayer.stats.map((s) => ({ ...s }))
    },
    { immediate: true },
)

// Update Player info, when the player in the backend changes
watch(
    () => playerStore.players,
    () => {
        const updatedPlayer = playerStore.getPlayerById(props.playerId)

        // When the player is not found (deleted), close the modal
        if (!updatedPlayer) {
            emit('done')
            return
        }

        playerNameRef.value = updatedPlayer.name
        playerSecretRef.value = updatedPlayer.secret
        playerStatsRef.value = updatedPlayer.stats.map((s) => ({ ...s }))
    },
    { deep: true },
)

function updatePlayer(): void {
    playerStore.updatePlayer(props.playerId, {
        name: playerNameRef.value,
        secret: playerSecretRef.value,
        stats: playerStatsRef.value,
    })
    emit('done')
}
</script>

<template>
    <h1 class="text-4xl text-center text-primary">Edit player</h1>
    <label class="input input-primary w-full">
        <span class="label">Name</span>
        <input type="text" v-model="playerNameRef" />
    </label>
    <label class="input input-primary w-full">
        <span class="label">Secret</span>
        <input type="text" v-model="playerSecretRef" />
    </label>
    <PlayerStatsEditor
        :player-id="props.playerId"
        :player-name="playerNameRef"
        :player-stats="playerStatsRef"
    />
    <button class="btn btn-primary btn-lg" @click="updatePlayer">Update Player</button>
</template>
