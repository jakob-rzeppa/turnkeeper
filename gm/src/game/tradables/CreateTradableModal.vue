<script setup lang="ts">
import { ref } from 'vue';
import { useCommandEmitter } from '../../commands/useCommandEmitter';

const commandEmitter = useCommandEmitter();

const tradableName = ref('');
const initialValue = ref(100);

const emit = defineEmits<{
    close: [];
}>();

const handleCreate = () => {
    if (!tradableName.value.trim()) {
        alert('Please enter a tradable name');
        return;
    }
    commandEmitter.addTradable(tradableName.value, initialValue.value);
    emit('close');
    tradableName.value = '';
    initialValue.value = 100;
};

const handleClose = () => {
    emit('close');
    tradableName.value = '';
    initialValue.value = 100;
};
</script>

<template>
    <form @submit.prevent="handleCreate" class="">
        <h2 class="text-xl font-bold mb-4">Create New Tradable</h2>
        <div class="mb-4">
            <label for="tradableName" class="block text-sm font-medium text-gray-700 mb-1"
                >Tradable Name</label
            >
            <input
                id="tradableName"
                v-model="tradableName"
                type="text"
                class="input input-bordered w-full"
                placeholder="Enter tradable name"
                required
            />
        </div>
        <div class="mb-4">
            <label for="initialValue" class="block text-sm font-medium text-gray-700 mb-1"
                >Initial Value</label
            >
            <input
                id="initialValue"
                v-model.number="initialValue"
                type="number"
                class="input input-bordered w-full"
                placeholder="Enter initial value"
                required
            />
        </div>
        <div class="flex gap-2">
            <button type="submit" class="btn btn-primary">Create</button>
            <button type="button" class="btn btn-outline" @click="handleClose">Cancel</button>
        </div>
    </form>
</template>
