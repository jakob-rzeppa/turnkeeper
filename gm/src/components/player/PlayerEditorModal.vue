<script setup lang="ts">
import PlayerEditor from './PlayerEditor.vue'
import { usePlayerEmitter } from '@/emitters/playerEmitter'

// The Player prop needs to be a deep clone
const props = defineProps<{
    playerId: number
}>()

const emit = defineEmits(['close'])

const playerEmitter = usePlayerEmitter()

function deletePlayer(): void {
    if (
        confirm(
            `Are you sure you want to delete player with id ${props.playerId}? This action cannot be undone.`,
        )
    ) {
        playerEmitter.deletePlayer(props.playerId)
        emit('close')
    }
}
</script>

<template>
    <div class="space-y-6">
        <PlayerEditor :playerId="props.playerId" @done="emit('close')" />
    </div>
</template>
