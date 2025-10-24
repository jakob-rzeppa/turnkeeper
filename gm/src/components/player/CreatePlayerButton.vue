<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';
import NewPlayerModal from './NewPlayerModal.vue';

const modalStore = useModalStore();

function openNewPlayerModal() {
    modalStore.openModal(NewPlayerModal);
}
</script>

<template>
    <button class="btn btn-secondary btn-sm w-full gap-2" @click="openNewPlayerModal">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
            ></path>
        </svg>
        Create Player
    </button>
</template>
