<script setup lang="ts">
import { useTurnStore } from '@/stores/turnStore'
import PlayerEditor from '../player/PlayerEditor.vue'
import DisplayContainer from '../DisplayContainer.vue'
import useConnection from '@/composables/connection'

const turnStore = useTurnStore()
const { socket } = useConnection()

socket.on(
    'game:turn',
    ({
        playerOrder,
        round,
    }: {
        playerOrder: { id: string; name: string }[]
        round: { roundNumber: number; currentPlayerIndex: number }
    }) => {
        turnStore.playerOrder = playerOrder
        turnStore.round = {
            roundNumber: round.roundNumber,
            currentPlayerIndex: round.currentPlayerIndex,
        }
    },
)

function endTurn() {
    socket.emit('game:turn:next')
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
