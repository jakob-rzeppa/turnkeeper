<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import BaseModal from '@/components/modal/BaseModal.vue'

const modalStore = useModalStore()
</script>

<template>
    <Teleport to="body">
        <BaseModal
            v-if="modalStore.topModal"
            :key="modalStore.topModal.id"
            :z-index="1000"
            @close="modalStore.closeTopModal"
        >
            <component :is="modalStore.topModal.component" v-bind="modalStore.topModal.props" />
        </BaseModal>
    </Teleport>
</template>
