<script setup lang="ts">
import { useGameStateStore } from '@/stores/gameStateStore';
import { usePlayerStore } from '@/stores/playerStore';

const gameStore = useGameStateStore();
const playerStore = usePlayerStore();
</script>

<template>
    <div class="divider">Game Status</div>

    <div v-if="gameStore.gameState" class="flex flex-col gap-2">
        <div
            v-if="
                gameStore.currentPlayer &&
                playerStore.player &&
                gameStore.currentPlayer.id === playerStore.player.id
            "
            class="p-4 bg-info text-info-content rounded-lg text-center font-semibold"
        >
            It's your turn!
        </div>
        <p><strong>Current Round:</strong> {{ gameStore.gameState.roundNumber }}</p>
        <p>
            <strong>Current Player:</strong>
            {{ gameStore.currentPlayer?.name ?? 'Unknown Player' }}
        </p>
        <div class="p-4 bg-base-200 rounded-lg">
            <pre class="whitespace-pre-wrap break-words text-sm">{{
                gameStore.gameState.notes
            }}</pre>
        </div>
    </div>
    <div v-else class="p-6 bg-base-200 rounded-lg shadow-md text-center">
        <p class="text-lg text-base-content/70">The game is not initialized yet.</p>
    </div>
</template>
