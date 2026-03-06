<script setup lang="ts">
import { ref } from 'vue';
import { API_BASE_URL, apiErrorToMessage } from '../api/httpApi';
import axios from 'axios';
import { useAuthStore } from '../auth/authStore';

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'lock'): void;
    (e: 'unlock'): void;
    (e: 'create'): void;
}>();

const authStore = useAuthStore();

const loading = ref(false);
const error = ref('');
const gameName = ref('');

async function handleCreateGameClick() {
    emit('lock');
    loading.value = true;
    try {
        await axios.post(
            API_BASE_URL + '/games',
            {
                name: gameName.value,
            },
            {
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            }
        );
        emit('create');
        emit('unlock');
        emit('close');
    } catch (e: unknown) {
        error.value = 'Failed to create game: ' + apiErrorToMessage(e);
    } finally {
        loading.value = false;
        emit('unlock');
    }
}
</script>

<template>
    <div class="p-4 space-y-4">
        <h2 class="text-xl font-bold text-center">
            Create New Game <span v-if="loading" class="loading loading-dots"></span>
        </h2>

        <label class="input w-full">
            <span class="label">Game Name</span>
            <input type="text" id="gameName" placeholder="Enter game name" v-model="gameName" />
        </label>

        <div v-if="error" class="alert alert-error">
            <strong>Error!</strong>
            <div>{{ error }}</div>
        </div>

        <div class="flex gap-4 w-full">
            <button
                @click="handleCreateGameClick"
                class="btn btn-primary flex-1"
                :disabled="loading"
            >
                Create Game
            </button>
            <button @click="emit('close')" class="btn btn-ghost flex-1" :disabled="loading">
                Cancel
            </button>
        </div>
    </div>
</template>
