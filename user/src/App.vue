<script setup lang="ts">
import LoginForm from './components/LoginForm.vue'
import PlayerDisplay from './components/PlayerDisplay.vue'
import useConnection from './composables/connection'
import { usePlayerStore } from './stores/playerStore'

const { isConnected } = useConnection()

// Initialize all stores, that define listeners for backend updates, so that they are ready when the user connects to the backend
usePlayerStore()
</script>

<template>
    <div>
        <LoginForm v-if="!isConnected" />
        <PlayerDisplay v-else />
    </div>
</template>
