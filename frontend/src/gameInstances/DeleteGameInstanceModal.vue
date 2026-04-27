<script setup lang="ts">
import { ref } from 'vue';
import { deleteWithAuth } from '../api/httpApi';

interface Props {
    gameId: string;
    gameInstanceId: string;
    gameInstanceName: string;
}

const props = defineProps<Props>();

const emit = defineEmits(['deleted', 'close']);

const isDeleting = ref(false);

const handleDelete = async () => {
    isDeleting.value = true;
    const res = await deleteWithAuth(
        '/games/' + props.gameId + '/instances/' + props.gameInstanceId
    );

    if (res.isOk()) {
        isDeleting.value = false;
        emit('deleted');
        emit('close');
    } else {
        alert(`Failed to delete game instance: ${res.error.message}`);
        isDeleting.value = false;
    }
};
</script>

<template>
    <h3 class="font-bold text-lg mb-4">Delete Game Instance</h3>

    <div class="alert alert-warning mb-6">
        <span
            >Are you sure you want to delete <strong>{{ gameInstanceName }}</strong
            >? This action cannot be undone.</span
        >
    </div>

    <div class="modal-action">
        <button @click="emit('close')" :disabled="isDeleting" class="btn btn-ghost">Cancel</button>
        <button @click="handleDelete" :disabled="isDeleting" class="btn btn-error">
            <span v-if="isDeleting" class="loading loading-spinner loading-sm"></span>
            <span v-else>Delete Game Instance</span>
        </button>
    </div>
</template>
