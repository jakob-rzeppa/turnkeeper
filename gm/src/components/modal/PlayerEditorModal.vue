<script setup lang="ts">
import type { Player } from '@/types/player'
import PlayerEditor from '../input/PlayerEditor.vue'
import { socket } from '@/util/connection'

// The Player prop needs to be a deep clone
const props = defineProps<{
    player: Player
}>()

const emit = defineEmits(['close'])

function deletePlayer(): void {
    if (
        confirm(
            `Are you sure you want to delete player ${props.player.name}? This action cannot be undone.`,
        )
    ) {
        socket.emit('players:delete', { playerId: props.player.id })
        emit('close')
    }
}
</script>

<template>
    <PlayerEditor :player="props.player" @done="emit('close')" />
    <button class="btn btn-error btn-sm btn-outline" @click="deletePlayer">Delete Player</button>
</template>
