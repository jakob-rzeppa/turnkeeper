<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import { shallowRef } from 'vue'
import NewPlayerModal from './modal/NewPlayerModal.vue'

const modalStore = useModalStore()

function openNewPlayerModal() {
    const newPlayerModal = shallowRef(NewPlayerModal)
    modalStore.openModal(newPlayerModal)
}
</script>

<template>
    <button class="btn btn-secondary" @click="openNewPlayerModal">Create Player</button>
</template>
