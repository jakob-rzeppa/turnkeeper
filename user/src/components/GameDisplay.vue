<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore'

const gameStore = useGameStore()
</script>

<template>
    <div class="divider">Game Status</div>
    <div v-if="gameStore.isInitialized">
        <p class="mb-2"><strong>Current Round:</strong> {{ gameStore.round.roundNumber }}</p>
        <p><strong>Current Player Index:</strong> {{ gameStore.round.currentPlayerIndex + 1 }}</p>
    </div>
    <div v-else class="p-6 bg-base-200 rounded-lg shadow-md text-center">
        <p class="text-lg text-base-content/70">The game is not initialized yet.</p>
    </div>
</template>
