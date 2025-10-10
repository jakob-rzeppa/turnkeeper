<script setup lang="ts">
import Login from './components/Login.vue'
import PlayerDisplay from './components/PlayerDisplay.vue'
import useConnection from './composables/connection'
import { usePlayerStore } from './stores/playerStore'

const { isConnected } = useConnection()

// Initialize all stores, that use listeners for backend updates
usePlayerStore()
</script>

<template>
    <Login v-if="!isConnected" />
    <PlayerDisplay v-else />
</template>

<style scoped></style>
