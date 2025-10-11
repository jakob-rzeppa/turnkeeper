<script setup lang="ts">
import DisplayContainer from '../container/DisplayContainer.vue'
import { useGameStore } from '@/stores/gameStore'

const gameStore = useGameStore()

function endTurn() {
    gameStore.nextTurn()
}
</script>

<template>
    <DisplayContainer label="Turn Management">
        <div v-if="gameStore.isInitialized" class="space-y-6">
            <!-- Round Info -->
            <div class="stats shadow-sm">
                <div class="stat">
                    <div class="stat-title">Current Round</div>
                    <div class="stat-value text-primary">{{ gameStore.round.roundNumber }}</div>
                </div>
            </div>

            <!-- Player Order Breadcrumbs -->
            <div class="breadcrumbs">
                <ul class="text-sm">
                    <li
                        v-for="(player, index) in gameStore.playerOrder"
                        :key="player.id"
                        class="flex items-center"
                    >
                        <div
                            class="flex items-center space-x-2"
                            :class="{
                                'bg-accent text-accent-content px-3 py-1 rounded-full font-bold':
                                    gameStore.currentPlayerId === player.id,
                                'text-base-content/70': gameStore.currentPlayerId !== player.id,
                            }"
                        >
                            <span
                                class="inline-flex items-center justify-center w-6 h-6 text-xs rounded-full bg-primary text-primary-content"
                            >
                                {{ index + 1 }}
                            </span>
                            <span>{{ player.name }}</span>
                        </div>
                    </li>
                </ul>
            </div>

            <!-- End Turn Button -->
            <button class="btn btn-accent btn-lg w-full" @click="endTurn">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M9 5l7 7-7 7"
                    ></path>
                </svg>
                End Turn
            </button>
        </div>
        <div v-else class="text-center py-8">
            <div class="alert alert-info">
                <svg class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                </svg>
                <span>Game not initialized. Use the drawer to start a new game.</span>
            </div>
        </div>
    </DisplayContainer>
</template>
