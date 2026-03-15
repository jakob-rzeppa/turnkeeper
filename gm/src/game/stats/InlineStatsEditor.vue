<script setup lang="ts">
import { useEventEmitter } from '../../events/useEventEmitter';
import type { Stat } from '../gameStore';

const props = defineProps<{
    playerId: string;
    stat: Stat;
}>();

const eventEmitter = useEventEmitter();

const editStatString = (newValue: string) => {
    eventEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'string', newValue.toString());
};
const editStatNumber = (newValue: number) => {
    eventEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'number', newValue.toString());
};
const editStatBoolean = (newValue: boolean) => {
    eventEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'boolean', newValue.toString());
};
</script>

<template>
    <div v-if="stat.valueType === 'boolean'" class="flex flex-row gap-2 pl-3">
        <label class="label">{{ stat.key }}</label>
        <input
            type="checkbox"
            class="toggle toggle-sm toggle-primary"
            :checked="stat.booleanValue!"
            @change="event => editStatBoolean((event.target as HTMLInputElement).checked)"
        />
    </div>
    <label v-else class="input input-sm">
        <span class="label">{{ stat.key }}</span>
        <input
            v-if="stat.valueType === 'string'"
            :value="stat.stringValue"
            @change="event => editStatString((event.target as HTMLInputElement).value)"
        />
        <input
            v-else-if="stat.valueType === 'number'"
            :value="stat.numberValue"
            @change="event => editStatNumber(parseFloat((event.target as HTMLInputElement).value))"
        />
    </label>
</template>
