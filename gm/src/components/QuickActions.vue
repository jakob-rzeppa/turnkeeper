<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';
import PlayerDisplayModal from './player/PlayerDisplayModal.vue';
import DisplayContainer from './container/DisplayContainer.vue';

const modalStore = useModalStore();
</script>

<template>
    <DisplayContainer label="Quick Actions">
        <button
            class="btn btn-sm btn-primary mb-4"
            @click="modalStore.openModal(PlayerDisplayModal, {}, '8xl')"
        >
            Open Player Display Modal
        </button>
    </DisplayContainer>
</template>
