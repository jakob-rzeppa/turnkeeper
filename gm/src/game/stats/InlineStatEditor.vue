<script setup lang="ts">
import { useCommandEmitter } from '../../commands/useCommandEmitter';
import type { Stat } from '../gameStore';

const props = defineProps<{
    playerId: string;
    stat: Stat;
    size?: 'sm' | 'md' | 'lg';
}>();

const commandEmitter = useCommandEmitter();

const editStatString = (newValue: string) => {
    commandEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'string', newValue.toString());
};
const editStatNumber = (newValue: number) => {
    commandEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'number', newValue.toString());
};
const editStatBoolean = (newValue: boolean) => {
    commandEmitter.changeStatOfPlayer(props.playerId, props.stat.id, 'boolean', newValue.toString());
};

const deleteStat = () => {
    commandEmitter.removeStatFromPlayer(props.playerId, props.stat.id);
};
</script>

<template>
    <div class="flex flex-row gap-2 items-center w-full">
        <div
            v-if="stat.valueType === 'boolean'"
            class="flex flex-row gap-2 pl-3 flex-1 items-center"
        >
            <label :class="`label text-${props.size || 'md'}`">{{ stat.key }}</label>
            <input
                type="checkbox"
                :class="`toggle toggle-${props.size || 'md'} toggle-accent`"
                :checked="stat.booleanValue!"
                @change="event => editStatBoolean((event.target as HTMLInputElement).checked)"
            />
        </div>
        <label v-else :class="`input input-${props.size || 'md'} flex-1`">
            <span class="label">
                {{ stat.key }}
                <span
                    :class="`badge badge-outline badge-${props.size || 'md'} badge-${stat.valueType === 'string' ? 'primary' : stat.valueType === 'number' ? 'secondary' : 'accent'}`"
                >
                    {{ stat.valueType }}
                </span>
            </span>
            <input
                v-if="stat.valueType === 'string'"
                :value="stat.stringValue"
                @change="event => editStatString((event.target as HTMLInputElement).value)"
            />
            <input
                v-else-if="stat.valueType === 'number'"
                :value="stat.numberValue"
                @change="
                    event => editStatNumber(parseFloat((event.target as HTMLInputElement).value))
                "
            />
        </label>
        <button
            @click="deleteStat"
            :class="`btn btn-error btn-ghost btn-${props.size || 'md'} btn-circle`"
        >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path
                    fill-rule="evenodd"
                    d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                    clip-rule="evenodd"
                />
            </svg>
        </button>
    </div>
</template>
