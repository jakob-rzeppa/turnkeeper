<script setup lang="ts">
import { ref } from 'vue'
import useConnection from '@/composables/connection'

const emit = defineEmits(['close'])

const { socket } = useConnection()

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
