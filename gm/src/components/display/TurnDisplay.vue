<script setup lang="ts">
import PlayerEditor from '../player/PlayerEditor.vue'
import DisplayContainer from '../DisplayContainer.vue'
import { useGameStore } from '@/stores/gameStore'

const gameStore = useGameStore()

function endTurn() {
    gameStore.nextTurn()
}
</script>

<template>
    <DisplayContainer label="Turn">
        <div v-if="gameStore.isInitialized" class="flex flex-col gap-4">
            <p>Round: {{ gameStore.round.roundNumber }}</p>
            <div class="breadcrumbs">
                <ul>
                    <li v-for="player in gameStore.playerOrder" :key="player.id">
                        <span
                            class="font-bold text-accent"
                            v-if="gameStore.currentPlayerId === player.id"
                        >
                            {{ player.name }}
                        </span>
                        <span v-else>{{ player.name }}</span>
                    </li>
                </ul>
            </div>
            <button class="btn btn-accent" @click="endTurn">End turn</button>
            <div v-if="gameStore.currentPlayerId">
                <PlayerEditor :playerId="gameStore.currentPlayerId" />
            </div>
            <div v-else>
                <p>No current player</p>
            </div>
        </div>
        <div v-else>Game not initialized</div>
    </DisplayContainer>
</template>
