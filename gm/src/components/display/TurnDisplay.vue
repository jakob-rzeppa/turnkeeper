<script setup lang="ts">
import { socket } from '@/util/connection'
import { computed, shallowRef } from 'vue'
import InitGameModal from '../modal/InitGameModal.vue'
import { useModalStore } from '@/stores/modalStore'
import { useTurnStore } from '@/stores/turnStore'
import PlayerEditor from '../input/PlayerEditor.vue'
import DisplayContainer from './DisplayContainer.vue'

const modalStore = useModalStore()
const turnStore = useTurnStore()

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

function openInitGameModal() {
    const initGameModal = shallowRef(InitGameModal)
    modalStore.openModal(initGameModal)
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-full" @click="openInitGameModal">Init Game</button>
    <DisplayContainer>
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
    </DisplayContainer>
</template>
