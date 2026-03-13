<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../gameStore';
import { useUsersStore } from '../../users/usersStore';
import PlayerStatsEditor from './PlayerStatsEditor.vue';
import PlayerTradablesEditor from './PlayerTradablesEditor.vue';

const gameStore = useGameStore();
const usersStore = useUsersStore();

const currentPlayer = computed(() => {
    const index = gameStore.game?.currentPlayerIndex;
    if (index === undefined || index < 0) return null;
    return gameStore.game?.players[index] ?? null;
});

const currentPlayerName = computed(() => {
    const player = currentPlayer.value;
    if (!player) return 'No Current Player';
    if (!player.userId) return 'Unassigned Player';
    return usersStore.getById(player.userId)?.value?.name ?? 'Name not found';
});
</script>

<template>
    <div>
        <h2 class="text-2xl font-bold">Round Overview</h2>
        <div>
            <p>Current Round: {{ gameStore.game?.roundNumber }}</p>
            <p>Current Player Index: {{ gameStore.game?.currentPlayerIndex }}</p>
            <p>Current Player: {{ currentPlayerName }}</p>
        </div>

        <h2 class="text-xl font-semibold mt-4">Edit current player stats</h2>
        <div class="divider">Tradables</div>
        <PlayerTradablesEditor v-if="currentPlayer" :player="currentPlayer" />
        <div class="divider">Stats</div>
        <PlayerStatsEditor v-if="currentPlayer" :player="currentPlayer" />
    </div>
</template>
