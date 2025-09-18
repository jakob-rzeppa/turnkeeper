<script setup lang="ts">
import { computed } from 'vue'

import { useModalStore } from '@/stores/modalStore'

const modalStore = useModalStore()

const topModal = computed(() => {
    const modals = modalStore.modals
    return modals.length > 0 ? modals[modals.length - 1] : null
})
</script>

<template>
    <Teleport to="body">
        <Transition name="modal-fade">
            <BaseModal
                v-if="topModal"
                :key="topModal.id"
                :z-index="1000"
                @close="modalStore.closeTopModal"
            >
                <component :is="topModal.component" v-bind="topModal.props" />
            </BaseModal>
        </Transition>
    </Teleport>
</template>
