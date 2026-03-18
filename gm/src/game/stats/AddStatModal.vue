<script setup lang="ts">
import { ref } from 'vue';
import { useCommandEmitter } from '../../commands/useCommandEmitter';

const props = defineProps<{
    playerId: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();

const commandEmitter = useCommandEmitter();

const newStatKey = ref('');
const newStatValueType = ref<'string' | 'number' | 'boolean'>('string');
const newStatValue = ref<string | number | boolean>('');

const addStat = () => {
    if (newStatKey.value.trim() === '') {
        return;
    }
    commandEmitter.addStatToPlayer(
        props.playerId,
        newStatKey.value,
        newStatValueType.value,
        newStatValue.value.toString()
    );
    newStatKey.value = '';
    newStatValueType.value = 'string';
    newStatValue.value = '';
    emit('close');
};
</script>

<template>
    <form @submit.prevent="addStat" class="flex flex-col gap-4">
        <h2 class="text-xl font-bold">Add Stat</h2>
        <label class="input">
            <span class="label">Key</span>
            <input v-model="newStatKey" placeholder="Stat Key" />
        </label>
        <label class="select">
            <span class="label">Value Type</span>
            <select v-model="newStatValueType">
                <option value="string">String</option>
                <option value="number">Number</option>
                <option value="boolean">Boolean</option>
            </select>
        </label>
        <div v-if="newStatValueType === 'boolean'" class="flex flex-row gap-2 ml-3">
            <span class="label">Value</span>
            <input v-model="newStatValue" type="checkbox" class="toggle" />
        </div>
        <label v-else class="input">
            <span class="label">Value</span>
            <input
                v-if="newStatValueType === 'string'"
                v-model="newStatValue"
                placeholder="Stat Value"
            />
            <input
                v-else-if="newStatValueType === 'number'"
                v-model.number="newStatValue"
                placeholder="Stat Value"
                type="number"
            />
        </label>

        <button type="submit" class="btn btn-primary">Add Stat</button>
    </form>
</template>
