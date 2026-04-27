<script setup lang="ts">
import { ref } from 'vue';
import { deleteGame } from '../api/requests/games/deleteGame';

interface Props {
    gameId: string;
    gameName: string;
}

const props = defineProps<Props>();

const emit = defineEmits(['close', 'delete']);

const isDeleting = ref(false);

const handleDelete = async () => {
    isDeleting.value = true;

    const res = await deleteGame(props.gameId);

    if (res.isOk()) {
        isDeleting.value = false;
        emit('delete');
        emit('close');
    } else {
        isDeleting.value = false;
        alert(`Failed to delete game: ${res.error}`);
    }
};
</script>

<template>
    <h3 class="font-bold text-lg mb-4">Delete Game</h3>

    <div class="alert alert-warning mb-6">
        <span
            >Are you sure you want to delete <strong>{{ gameName }}</strong
            >? This action cannot be undone.
        </span>
    </div>

    <div class="modal-action">
        <button @click="emit('close')" :disabled="isDeleting" class="btn btn-ghost">Cancel</button>
        <button @click="handleDelete" :disabled="isDeleting" class="btn btn-error">
            <span v-if="isDeleting" class="loading loading-spinner loading-sm"></span>
            <span v-else>Delete Game</span>
        </button>
    </div>
</template>
