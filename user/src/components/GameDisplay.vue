<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore'

const gameStore = useGameStore()
</script>

<template>
    <div>
        <div v-if="gameStore.isInitialized" class="p-6 bg-base-200 rounded-lg shadow-md">
            <h2 class="text-2xl font-bold mb-4 text-primary">Game Status</h2>
            <p class="mb-2"><strong>Current Round:</strong> {{ gameStore.round.roundNumber }}</p>
            <p>
                <strong>Current Player Index:</strong> {{ gameStore.round.currentPlayerIndex + 1 }}
            </p>
        </div>
        <div v-else class="p-6 bg-base-200 rounded-lg shadow-md text-center">
            <p class="text-lg text-base-content/70">
                No game initialized. Please start a new game.
            </p>
        </div>
    </div>
</template>
