<script setup lang="ts">
import { socket } from '@/util/connection'
import { ref, shallowRef } from 'vue'
import InitGameModal from './modal/InitGameModal.vue'
import { useModalStore } from '@/stores/modalStore'
import { usePlayerStore } from '@/stores/playerStore'
import { useTurnStore } from '@/stores/turnStore'

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
        <div class="breadcrumbs">
            <ul>
                <li v-for="player in playerStore.players" :key="player.id">{{ player.name }}</li>
            </ul>
        </div>
        <div>
            <p>Round: {{ turnStore.round.roundNumber }}</p>
            <p>
                Current Player:
                {{ turnStore.playerOrder[turnStore.round.currentPlayerIndex]?.name ?? 'N/A' }}
            </p>
        </div>
    </div>
</template>
