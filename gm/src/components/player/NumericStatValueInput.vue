<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
    class: string;
    statId: string;
    baseStats: {
        [key: string]: {
            name: string;
            type: 'string' | 'number' | 'boolean';
            value: string | number | boolean;
        };
    };
    editableStats: {
        [key: string]: {
            name: string;
            type: 'string' | 'number' | 'boolean';
            value: string | number | boolean;
        };
    };
}>();

const inputValue = ref<string>('');

// Watch for changes in the baseStats prop to update the inputValue
watch(
    () => props.baseStats[props.statId]?.value,
    (newValue) => {
        inputValue.value = newValue?.toString() ?? '0';
    },
    { immediate: true },
);

// Watch for changes in the inputValue to update the model prop
watch(
    () => inputValue.value,
    (newValue) => {
        let newValueAsNumber = parseFloat(newValue);

        if (isNaN(newValueAsNumber)) {
            newValueAsNumber = 0;
        }

        if (props.editableStats[props.statId]) {
            props.editableStats[props.statId].value = newValueAsNumber;
        }
    },
    { immediate: true },
);
</script>

<template>
    <input type="text" :class="props.class" v-model="inputValue" />
</template>
