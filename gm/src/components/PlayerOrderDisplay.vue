<script setup lang="ts">
import { socket } from '@/util/connection'
import { ref, shallowRef } from 'vue'
import InitGameModal from './modal/InitGameModal.vue'
import { useModalStore } from '@/stores/modalStore'

const modalStore = useModalStore()

const playerOrderRef = ref<{ id: string; name: string }[]>([])

socket.on('game:turn:players:order', (data: { playerOrder: { id: string; name: string }[] }) => {
    playerOrderRef.value = data.playerOrder
})

function openInitGameModal() {
    const initGameModal = shallowRef(InitGameModal)
    modalStore.openModal(initGameModal)
}
</script>

<template>
    <div>
        <div class="breadcrumbs">
            <ul>
                <li v-for="player in playerOrderRef" :key="player.id">{{ player.name }}</li>
            </ul>
        </div>
        <button class="btn btn-primary btn-sm w-fit" @click="openInitGameModal">Init Game</button>
    </div>
</template>
