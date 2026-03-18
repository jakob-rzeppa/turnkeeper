<script setup lang="ts">
import { useModalStore } from './modalStore';
import BaseModal from './BaseModal.vue';
import { ref } from 'vue';

/*
 * This component is responsible for rendering the topmost modal from the modal store.
 */

// The locked state can be used by modals to prevent closing when clicking outside or pressing escape.
const locked = ref(false);

const modalStore = useModalStore();

function closeTopModal() {
    if (!locked.value) {
        modalStore.closeTopModal();
    }
}
</script>

<template>
    <Teleport to="body">
        <BaseModal
            v-if="modalStore.topModal"
            :key="modalStore.topModal.id"
            :z-index="1000"
            :width="modalStore.topModal.width"
            @close="closeTopModal"
        >
            <component
                :is="modalStore.topModal.component"
                v-bind="modalStore.topModal.props"
                v-on="modalStore.topModal.emits"
                @close="closeTopModal"
                @lock="locked = true"
                @unlock="locked = false"
            />
        </BaseModal>
    </Teleport>
</template>
