<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import { ref } from 'vue'

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()

const newPlayerNameRef = ref('')

function createPlayer() {
    playerStore.createPlayer(newPlayerNameRef.value)
    emit('close')
}
</script>

<template>
    <h1 class="text-4xl text-center text-primary">Create New Player</h1>
    <label class="input">
        <span>Player Name</span>
        <input type="text" v-model="newPlayerNameRef" />
    </label>
    <button class="btn btn-primary" @click="createPlayer">Create Player</button>
</template>
