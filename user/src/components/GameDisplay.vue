<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore'
import { usePlayerStore } from '@/stores/playerStore'

const gameStore = useGameStore()
const playerStore = usePlayerStore()
</script>

<template>
    <div class="divider">Game Status</div>

    <div v-if="gameStore.isInitialized && gameStore.playerOrder.length > 0">
        <div
            v-if="
                gameStore.currentPlayer &&
                playerStore.player &&
                gameStore.currentPlayer.id === playerStore.player.id
            "
            class="p-4 mb-4 bg-info text-info-content rounded-lg text-center font-semibold"
        >
            It's your turn!
        </div>
        <p class="mb-2"><strong>Current Round:</strong> {{ gameStore.round.roundNumber }}</p>
        <p>
            <strong>Current Player:</strong>
            {{ gameStore.currentPlayer?.name ?? 'Unknown Player' }}
        </p>
    </div>
    <div v-else class="p-6 bg-base-200 rounded-lg shadow-md text-center">
        <p class="text-lg text-base-content/70">The game is not initialized yet.</p>
    </div>
</template>
