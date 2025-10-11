<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import NewPlayerModal from './NewPlayerModal.vue'

const modalStore = useModalStore()

function openNewPlayerModal() {
    modalStore.openModal(NewPlayerModal)
}
</script>

<template>
    <button class="btn btn-secondary" @click="openNewPlayerModal">Create Player</button>
</template>
