<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import type { Player } from '../gameStore';
import { useEventEmitter } from '../../events/useEventEmitter';
import { useModalStore } from '../../common/modal/modalStore';
import SendTradableModal from './SendTradableModal.vue';

const props = defineProps<{
    player: Player;
}>();

const eventEmitter = useEventEmitter();
const modalStore = useModalStore();

const tradables = computed(() => props.player.tradables);
const editableValues = ref<Record<string, number>>({});

watch(
    tradables,
    nextTradables => {
        const nextValues: Record<string, number> = {};
        nextTradables.forEach(tradable => {
            nextValues[tradable.id] = tradable.value;
        });
        editableValues.value = nextValues;
    },
    { immediate: true }
);

const commitTradableValue = (tradableId: string) => {
    const newValue = editableValues.value[tradableId];
    if (Number.isNaN(newValue)) {
        return;
    }

    eventEmitter.changePlayerTradableValue(props.player.id, tradableId, newValue ?? 0);
};
</script>

<template>
    <div class="space-y-4">
        <div v-if="tradables.length === 0" class="alert alert-info">
            <span>No tradables for this player.</span>
        </div>
        <div v-else class="space-y-2">
            <div
                v-for="tradable in tradables"
                :key="tradable.id"
                class="bg-base-200 hover:bg-base-300 transition-colors rounded-lg px-4 py-3 flex items-center justify-between gap-4"
            >
                <span class="font-semibold text-base flex-1">{{ tradable.name }}</span>
                <div class="flex items-center gap-2">
                    <input
                        v-model.number="editableValues[tradable.id]"
                        type="number"
                        class="input input-sm input-bordered w-24 text-right"
                        @change="commitTradableValue(tradable.id)"
                    />
                    <button
                        class="btn btn-sm btn-square btn-ghost btn-primary"
                        title="Send tradable"
                        aria-label="Send tradable"
                        @click="
                            modalStore.openModal(SendTradableModal, {
                                tradableId: tradable.id,
                                tradableName: tradable.name,
                                fromPlayerId: props.player.id,
                                playerTradableValue: editableValues[tradable.id] ?? 0,
                            })
                        "
                    >
                        <svg
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-4 w-4"
                        >
                            <path d="M22 2L11 13" />
                            <path d="M22 2L15 22l-4-9-9-4 20-7z" />
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
