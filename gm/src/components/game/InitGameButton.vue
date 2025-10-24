<script setup lang="ts">
import InitGameModal from './InitGameModal.vue';
import { useModalStore } from '@/stores/modalStore';

const modalStore = useModalStore();

function openInitGameModal() {
    modalStore.openModal(InitGameModal);
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-full" @click="openInitGameModal">Init Game</button>
</template>
