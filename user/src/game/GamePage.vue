<script setup lang="ts">
import { useGameStore } from './gameStore';

const gameStore = useGameStore();
</script>

<template>
    <div v-if="gameStore.game" class="flex flex-col gap-4 items-center">
        <h1 class="text-2xl">{{ gameStore.game.name }}</h1>
        <p class="text-lg">Current Round: {{ gameStore.game.roundNumber }}</p>

        <p>Notes</p>
        <pre>{{ gameStore.game.notes }}</pre>

        <div class="divider"></div>

        <div v-if="gameStore.game.ownPlayer">
            <p class="text-lg">Your Role: {{ gameStore.game.ownPlayer.stats }}</p>
        </div>
        <div v-else>
            <p class="text-lg">You have no player assigned.</p>
        </div>
    </div>
    <div v-else>
        <h1 class="display-2 text-center">Waiting for game...</h1>
    </div>
</template>
