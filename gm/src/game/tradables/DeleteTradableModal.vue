<script setup lang="ts">
import { useCommandEmitter } from '../../commands/useCommandEmitter';

const props = defineProps<{
    tradableId: string;
    tradableName: string;
}>();

const commandEmitter = useCommandEmitter();

const emit = defineEmits<{
    close: [];
}>();

const handleDelete = () => {
    commandEmitter.removeTradable(props.tradableId);
    emit('close');
};
</script>

<template>
    <form @submit.prevent="handleDelete">
        <h2 class="text-xl font-bold mb-4">Delete Tradable</h2>
        <p class="mb-4">Are you sure you want to delete the tradable "{{ tradableName }}"?</p>
        <div class="flex space-x-2">
            <button type="submit" class="btn btn-error btn-outline">Delete</button>
            <button type="button" @click="emit('close')" class="btn btn-primary">Cancel</button>
        </div>
    </form>
</template>
