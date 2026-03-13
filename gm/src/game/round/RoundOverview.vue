<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../gameStore';
import { useUsersStore } from '../../users/usersStore';
import PlayerStatsEditor from './PlayerStatsEditor.vue';
import PlayerTradablesEditor from './PlayerTradablesEditor.vue';
import { useEventEmitter } from '../../events/useEventEmitter';

const gameStore = useGameStore();
const usersStore = useUsersStore();
const eventEmitter = useEventEmitter();

const currentPlayer = computed(() => {
    const index = gameStore.game?.currentPlayerIndex;
    if (index === undefined || index < 0) return null;
    return gameStore.game?.players[index] ?? null;
});
</script>

<template>
    <h2 class="text-2xl font-bold">Round Overview</h2>
    <div class="flex flex-col gap-4">
        <div>
            <p>Round: {{ gameStore.game?.roundNumber }}</p>
        </div>
        <div class="flex flex-row gap-2 items-center">
            <template v-for="(player, index) in gameStore.game?.players" :key="player.id">
                <button
                    :class="[
                        'btn btn-xs',
                        currentPlayer?.id !== player.id ? 'btn-ghost' : 'btn-secondary',
                    ]"
                    @click="eventEmitter.skipTurnToPlayer(player.id)"
                >
                    {{ usersStore.getPlayerName(player.id) }}
                </button>
                <span v-if="index < (gameStore.game?.players?.length ?? 0) - 1" class="text-lg">
                    →
                </span>
            </template>
        </div>
        <div class="flex flex-row gap-2">
            <button class="btn btn-sm btn-ghost btn-warning" @click="eventEmitter.previousTurn">
                Previous Turn
            </button>
            <button class="btn btn-sm btn-primary" @click="eventEmitter.nextTurn">Next Turn</button>
        </div>
    </div>

    <div class="divider">Tradables</div>
    <PlayerTradablesEditor v-if="currentPlayer" :player="currentPlayer" />
    <div class="divider">Stats</div>
    <PlayerStatsEditor v-if="currentPlayer" :player="currentPlayer" />
</template>
