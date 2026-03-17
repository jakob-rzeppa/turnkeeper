<script setup lang="ts">
import { useModalStore } from '../../common/modal/modalStore';
import { useCommandEmitter } from '../../commands/useCommandEmitter';
import type { Tradable } from '../gameStore';
import SendTradableModal from './SendTradableModal.vue';

const props = defineProps<{
    playerId: string;
    tradable: Tradable;
    size?: 'sm' | 'md' | 'lg';
}>();

const commandEmitter = useCommandEmitter();
const modalStore = useModalStore();

const editTradable = (newValue: number) => {
    commandEmitter.changePlayerTradableValue(props.playerId, props.tradable.id, newValue);
};
</script>

<template>
    <div class="flex flex-row gap-2 items-center">
        <label :class="`input input-${props.size || 'md'} flex-1`">
            <span class="label">{{ tradable.name }}</span>
            <input
                type="number"
                :value="props.tradable.value"
                @change="
                    event => editTradable(parseFloat((event.target as HTMLInputElement).value))
                "
            />
        </label>
        <button
            :class="`btn btn-circle btn-ghost btn-${props.size || 'md'} btn-primary`"
            title="Send tradable"
            aria-label="Send tradable"
            @click="
                modalStore.openModal(SendTradableModal, {
                    tradableId: tradable.id,
                    tradableName: tradable.name,
                    fromPlayerId: props.playerId,
                    playerTradableValue: props.tradable.value,
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
</template>
