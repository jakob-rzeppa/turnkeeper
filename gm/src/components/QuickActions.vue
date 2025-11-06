<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';
import PlayerDisplayModal from './player/PlayerDisplayModal.vue';
import DisplayContainer from './container/DisplayContainer.vue';
import NewStatModal from './player/NewStatModal.vue';
import MessagesModal from './messages/MessagesModal.vue';

const modalStore = useModalStore();
</script>

<template>
    <DisplayContainer label="Quick Actions">
        <div class="flex flex-wrap gap-4 justify-center">
            <button
                class="btn btn-sm btn-primary"
                @click="modalStore.openModal(PlayerDisplayModal, {}, '8xl')"
            >
                Open Player Display Modal
            </button>
            <button class="btn btn-sm btn-secondary" @click="modalStore.openModal(NewStatModal)">
                Create Stat for all Players
            </button>
            <button
                class="btn btn-sm btn-tertiary"
                @click="modalStore.openModal(MessagesModal, {}, '8xl')"
            >
                View Messages
            </button>
        </div>
    </DisplayContainer>
</template>
