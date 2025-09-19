<script setup lang="ts">
const props = defineProps<{
    playerId?: string
    playerName?: string
}>()
</script>

<template>
    <h1 class="text-4xl text-center text-primary">Add new stat</h1>
    <label class="select select-primary">
        <span class="label">Scope</span>
        <select>
            <option value="global" selected>Global</option>
            <option value="player" :disabled="!props.playerId">
                Player {{ props.playerName ?? '(unknown)' }}
            </option>
        </select>
    </label>
    <label class="input input-primary">
        <span class="label">Name</span>
        <input type="text" />
    </label>
    <label class="input input-primary">
        <span class="label">Value</span>
        <input type="text" />
    </label>
    <button class="btn btn-primary">Add Stat</button>
</template>
