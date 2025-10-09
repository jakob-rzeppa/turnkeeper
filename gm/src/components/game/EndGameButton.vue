<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore'

const gameStore = useGameStore()

function endGame() {
    confirm('Are you sure you want to end the game? This action cannot be undone.') &&
        gameStore.endGame()
}
</script>

<template>
    <button class="btn btn-error btn-outline btn-sm w-full" @click="endGame">End Game</button>
</template>
