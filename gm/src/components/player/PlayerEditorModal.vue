<script setup lang="ts">
import PlayerEditor from './PlayerEditor.vue'

// The Player prop needs to be a deep clone
const props = defineProps<{
    playerId: number
}>()

const emit = defineEmits(['close'])
</script>

<template>
    <div class="space-y-6">
        <PlayerEditor :playerId="props.playerId" @done="emit('close')" />
    </div>
</template>
