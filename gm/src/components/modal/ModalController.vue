<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore';
import BaseModal from '@/components/modal/BaseModal.vue';

/*
 * This component is responsible for rendering the topmost modal from the modal store.
 */

const modalStore = useModalStore();
</script>

<template>
    <Teleport to="body">
        <BaseModal
            v-if="modalStore.topModal"
            :key="modalStore.topModal.id"
            :z-index="1000"
            @close="modalStore.closeTopModal"
        >
            <component
                :is="modalStore.topModal.component"
                v-bind="modalStore.topModal.props"
                @close="modalStore.closeTopModal"
            />
        </BaseModal>
    </Teleport>
</template>
