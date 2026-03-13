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
</script>

<template>
    <div>
        <h2 class="text-2xl font-bold">Round Overview</h2>
        <div>
            <p>Round: {{ gameStore.game?.roundNumber }}</p>
        </div>
        <div class="flex flex-row gap-2 items-center">
            <template v-for="(player, index) in gameStore.game?.players" :key="player.id">
                <div
                    :class="[
                        'badge badge-lg',
                        currentPlayer?.id !== player.id ? 'badge-outline' : 'badge-primary',
                    ]"
                >
                    {{ usersStore.getPlayerName(player.id) }}
                </div>
                <span v-if="index < (gameStore.game?.players?.length ?? 0) - 1" class="text-lg">
                    →
                </span>
            </template>
        </div>

        <div class="divider">Tradables</div>
        <PlayerTradablesEditor v-if="currentPlayer" :player="currentPlayer" />
        <div class="divider">Stats</div>
        <PlayerStatsEditor v-if="currentPlayer" :player="currentPlayer" />
    </div>
</template>
