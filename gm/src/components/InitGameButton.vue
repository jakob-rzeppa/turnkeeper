<script setup lang="ts">
import { shallowRef } from 'vue'
import InitGameModal from './modal/InitGameModal.vue'
import { useModalStore } from '@/stores/modalStore'

const modalStore = useModalStore()

function openInitGameModal() {
    const initGameModal = shallowRef(InitGameModal)
    modalStore.openModal(initGameModal)
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-full" @click="openInitGameModal">Init Game</button>
</template>
