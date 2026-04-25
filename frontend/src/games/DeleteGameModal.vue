<script setup lang="ts">
import { ref } from 'vue';
import { useGamesStore } from './useGamesStore';

interface Props {
    gameId: string;
    gameName: string;
}

const props = defineProps<Props>();

const emit = defineEmits(['close']);

const gamesStore = useGamesStore();
const isDeleting = ref(false);

const handleDelete = async () => {
    isDeleting.value = true;
    try {
        await gamesStore.deleteGame(props.gameId);
        emit('close');
    } finally {
        isDeleting.value = false;
    }
};
</script>

<template>
    <div class="modal modal-open">
        <div class="modal-box max-w-md">
            <h3 class="font-bold text-lg mb-4">Delete Game</h3>

            <div class="alert alert-warning mb-6">
                <span
                    >Are you sure you want to delete <strong>{{ gameName }}</strong
                    >? This action cannot be undone.</span
                >
            </div>

            <div class="modal-action">
                <button @click="emit('close')" :disabled="isDeleting" class="btn btn-ghost">
                    Cancel
                </button>
                <button @click="handleDelete" :disabled="isDeleting" class="btn btn-error">
                    <span v-if="isDeleting" class="loading loading-spinner loading-sm"></span>
                    <span v-else>Delete Game</span>
                </button>
            </div>
        </div>
        <div class="modal-backdrop" @click="emit('close')"></div>
    </div>
</template>
