<script setup lang="ts">
import { useEventEmitter } from '../../events/useEventEmitter';
import type { Tradable } from '../gameStore';

const props = defineProps<{
    playerId: string;
    tradable: Tradable;
}>();

const eventEmitter = useEventEmitter();

const editTradable = (newValue: number) => {
    eventEmitter.changePlayerTradableValue(props.playerId, props.tradable.id, newValue);
};
</script>

<template>
    <label class="input input-sm">
        <span class="label">{{ tradable.name }}</span>
        <input
            type="number"
            :value="props.tradable.value"
            @change="event => editTradable(parseFloat((event.target as HTMLInputElement).value))"
        />
    </label>
</template>
