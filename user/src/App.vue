<script setup lang="ts">
import GameDisplay from './components/GameDisplay.vue'
import LoginForm from './components/LoginForm.vue'
import PlayerDisplay from './components/PlayerDisplay.vue'
import useConnection from './composables/connection'
import { useGameStore } from './stores/gameStore'
import { usePlayerStore } from './stores/playerStore'

const { isConnected } = useConnection()

// Initialize all stores, that define listeners for backend updates, so that they are ready when the user connects to the backend
usePlayerStore()
useGameStore()
</script>

<template>
    <div>
        <LoginForm v-if="!isConnected" />
        <div v-else>
            <GameDisplay />
            <PlayerDisplay />
        </div>
    </div>
</template>
