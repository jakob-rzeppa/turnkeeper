<script setup lang="ts">
import DisplayContainer from '../container/DisplayContainer.vue'
import { useGameStore } from '@/stores/gameStore'
import { usePlayerEditor } from '@/composables/usePlayerEditor'
import PlayerStatsEditor from '../player/PlayerStatsEditor.vue'

const gameStore = useGameStore()

const { localPlayer, updatePlayer } = usePlayerEditor()
</script>

<template>
    <DisplayContainer label="Current Player">
        <div v-if="gameStore.isInitialized">
            <div v-if="gameStore.currentPlayerId" class="flex flex-col gap-4">
                <h1 class="text-3xl font-bold text-primary">{{ localPlayer.name }}</h1>

                <PlayerStatsEditor
                    :player-id="gameStore.currentPlayerId"
                    :player-name="localPlayer.name"
                    :player-stats="localPlayer.stats"
                />

                <button class="btn btn-primary btn-lg w-full" @click="updatePlayer">
                    <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M5 13l4 4L19 7"
                        ></path>
                    </svg>
                    Update Player
                </button>
            </div>
            <div v-else class="alert alert-warning">
                <svg class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"
                    ></path>
                </svg>
                <span>No current player selected</span>
            </div>
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
