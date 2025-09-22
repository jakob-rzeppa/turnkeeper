<script setup lang="ts">
import { socket } from '@/util/connection'
import { computed, shallowRef } from 'vue'
import InitGameModal from './modal/InitGameModal.vue'
import { useModalStore } from '@/stores/modalStore'
import { usePlayerStore } from '@/stores/playerStore'
import { useTurnStore } from '@/stores/turnStore'
import PlayerEditor from './input/PlayerEditor.vue'

const modalStore = useModalStore()
const playerStore = usePlayerStore()
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

function openInitGameModal() {
    const initGameModal = shallowRef(InitGameModal)
    modalStore.openModal(initGameModal)
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-fit" @click="openInitGameModal">Init Game</button>
    <div class="p-4 border rounded-sm bg-base-200">
        <p>Round: {{ turnStore.round.roundNumber }}</p>
        <div class="breadcrumbs">
            <ul>
                <li v-for="player in turnStore.playerOrder" :key="player.id">{{ player.name }}</li>
            </ul>
        </div>
        <div v-if="turnStore.currentPlayerId">
            <PlayerEditor :playerId="turnStore.currentPlayerId" />
        </div>
        <div v-else>
            <p>No current player</p>
        </div>
    </div>
</template>
