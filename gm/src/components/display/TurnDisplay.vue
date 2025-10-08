<script setup lang="ts">
import { useTurnStore } from '@/stores/turnStore'
import PlayerEditor from '../player/PlayerEditor.vue'
import DisplayContainer from '../DisplayContainer.vue'

const turnStore = useTurnStore()

function endTurn() {
    turnStore.nextTurn()
}
</script>

<template>
    <DisplayContainer label="Turn">
        <div v-if="turnStore.isInitialized" class="flex flex-col gap-4">
            <p>Round: {{ turnStore.round.roundNumber }}</p>
            <div class="breadcrumbs">
                <ul>
                    <li v-for="player in turnStore.playerOrder" :key="player.id">
                        <span
                            class="font-bold text-accent"
                            v-if="turnStore.currentPlayerId === player.id"
                        >
                            {{ player.name }}
                        </span>
                        <span v-else>{{ player.name }}</span>
                    </li>
                </ul>
            </div>
            <button class="btn btn-accent" @click="endTurn">End turn</button>
            <div v-if="turnStore.currentPlayerId">
                <PlayerEditor :playerId="turnStore.currentPlayerId" />
            </div>
            <div v-else>
                <p>No current player</p>
            </div>
        </div>
        <div v-else>Game not initialized</div>
    </DisplayContainer>
</template>
