<script setup lang="ts">
import { socket } from '@/util/connection'
import { ref } from 'vue'

const emit = defineEmits(['close'])

const newPlayerNameRef = ref('')

function createPlayer() {
    if (newPlayerNameRef.value.trim()) {
        socket.emit('players:create', { name: newPlayerNameRef.value.trim() })
        emit('close')
    }
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
