<script setup lang="ts">
import { shallowRef } from 'vue'
import { useModalStore } from '@/stores/modalStore'
import UpdatePlayerOrderModal from './modal/UpdatePlayerOrderModal.vue'

const modalStore = useModalStore()

function openUpdatePlayerOrderModal() {
    const updatePlayerOrderModal = shallowRef(UpdatePlayerOrderModal)
    modalStore.openModal(updatePlayerOrderModal)
}
</script>

<template>
    <button class="btn btn-primary btn-sm w-full" @click="openUpdatePlayerOrderModal">
        Update Player Order
    </button>
</template>
