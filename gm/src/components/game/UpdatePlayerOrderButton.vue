<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import UpdatePlayerOrderModal from './UpdatePlayerOrderModal.vue'

const modalStore = useModalStore()

function openUpdatePlayerOrderModal() {
    modalStore.openModal(UpdatePlayerOrderModal)
}
</script>

<template>
    <button class="btn btn-accent btn-sm w-full gap-2" @click="openUpdatePlayerOrderModal">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 10h16M4 14h16M4 18h16"
            ></path>
        </svg>
        Update Order
    </button>
</template>
