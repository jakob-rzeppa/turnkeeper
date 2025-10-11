<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import UpdatePlayerOrderModal from '../player/UpdatePlayerOrderModal.vue'

const modalStore = useModalStore()

function openUpdatePlayerOrderModal() {
    modalStore.openModal(UpdatePlayerOrderModal)
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-full" @click="openUpdatePlayerOrderModal">
        Update Player Order
    </button>
</template>
