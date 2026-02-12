<script setup lang="ts">
const emit = defineEmits<{
    (e: 'close'): void;
}>();

function handleCreateGameClick() {
    emit('close');
}
</script>

<template>
    <div class="p-4 space-y-4">
        <h2 class="text-xl font-bold text-center">Create New Game</h2>

        <label class="input w-full">
            <span class="label">Game Name</span>
            <input type="text" id="gameName" placeholder="Enter game name" />
        </label>

        <div class="flex gap-4 w-full">
            <button @click="handleCreateGameClick" class="btn btn-primary flex-1">
                Create Game
            </button>
            <button @click="emit('close')" class="btn btn-ghost flex-1">Cancel</button>
        </div>
    </div>
</template>
