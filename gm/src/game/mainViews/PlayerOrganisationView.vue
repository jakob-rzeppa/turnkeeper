<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../../game/gameStore';
import { useWsConnection } from '../../api/useWsConnection';

const wsConnection = useWsConnection();
const gameStore = useGameStore();
const players = computed(() => gameStore.game?.players ?? []);
const currentPlayerIndex = computed(() => gameStore.game?.currentPlayerIndex ?? -1);
</script>

<template>
    <div class="w-full h-full p-4 overflow-y-scroll">
        <h2 class="text-lg font-semibold mb-3">Players</h2>
        <ul v-if="players.length > 0" class="flex flex-col gap-2">
            <li
                v-for="(player, index) in players"
                :key="player.id"
                class="flex items-center gap-3 p-3 rounded-lg border bg-base-200"
                :class="index === currentPlayerIndex ? 'border-accent' : 'border-base-300'"
            >
                <span class="text-sm font-medium text-gray-500 w-5 text-right">{{
                    index + 1
                }}</span>
                <span class="font-medium flex-1">{{ player.name }}</span>
                <span
                    v-if="index === currentPlayerIndex"
                    class="text-xs px-2 py-0.5 rounded-full bg-accent text-accent-content"
                >
                    Active
                </span>
            </li>
        </ul>
        <p v-else class="text-sm text-center">No players yet.</p>
        <button
            class="btn btn-primary btn-circle mx-auto mt-2 block"
            @click="wsConnection.send('ADD_PLAYER')"
        >
            +
        </button>
    </div>
</template>
