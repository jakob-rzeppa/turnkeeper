<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import PlayerEditor from './PlayerEditor.vue'

// The Player prop needs to be a deep clone
const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()

function deletePlayer(): void {
    if (
        confirm(
            `Are you sure you want to delete player with id ${props.playerId}? This action cannot be undone.`,
        )
    ) {
        playerStore.deletePlayer(props.playerId)
        emit('close')
    }
}
</script>

<template>
    <PlayerEditor :playerId="props.playerId" @done="emit('close')" />
    <button class="btn btn-error btn-sm btn-outline" @click="deletePlayer">Delete Player</button>
</template>
