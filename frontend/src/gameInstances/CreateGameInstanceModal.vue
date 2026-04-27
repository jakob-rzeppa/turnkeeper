<script setup lang="ts">
import { ref } from 'vue';
import { createGameInstance } from '../api/requests/gameInstances/createGameInstance';

const props = defineProps<{
    gameId: string;
    gameName: string;
}>();

const emit = defineEmits(['created', 'close']);

const name = ref('');

const handleCreateGameInstance = async () => {
    const gameInstanceName = name.value.trim();

    const res = await createGameInstance(props.gameId, gameInstanceName);

    if (res.isOk()) {
        emit('created');
        emit('close');
    } else {
        alert(`Failed to create game instance: ${res.error}`);
    }
};
</script>

<template>
    <h3 class="font-bold text-lg mb-6">Create New Game Instance for "{{ gameName }}"</h3>

    <form @submit.prevent="handleCreateGameInstance" class="space-y-4">
        <div class="form-control w-full">
            <label class="label">
                <span class="label-text font-semibold">Game Instance Name</span>
            </label>
            <input
                type="text"
                placeholder="Enter game instance name"
                v-model="name"
                class="input input-bordered w-full"
                required
            />
            <button type="submit" class="btn btn-primary mt-4">Create</button>
        </div>
    </form>
</template>
