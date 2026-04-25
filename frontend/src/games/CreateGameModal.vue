<script setup lang="ts">
import { ref } from 'vue';
import { useGamesStore } from './useGamesStore';

const emit = defineEmits(['close']);

const gamesStore = useGamesStore();

const name = ref('');
const description = ref('');

const submit = async () => {
    await gamesStore.createGame(name.value, description.value);
    emit('close');
};
</script>

<template>
    <div class="modal modal-open">
        <div class="modal-box max-w-md">
            <h3 class="font-bold text-lg mb-6">Create New Game</h3>

            <form @submit.prevent="submit" class="space-y-4">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text font-semibold">Game Name</span>
                    </label>
                    <input
                        type="text"
                        placeholder="Enter game name"
                        v-model="name"
                        class="input input-bordered w-full"
                        required
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text font-semibold">Description</span>
                    </label>
                    <textarea
                        placeholder="Enter game description"
                        v-model="description"
                        class="textarea textarea-bordered w-full h-24"
                        required
                    ></textarea>
                </div>

                <div class="modal-action mt-6">
                    <button type="button" @click="emit('close')" class="btn btn-ghost">
                        Cancel
                    </button>
                    <button type="submit" class="btn btn-primary">Create Game</button>
                </div>
            </form>
        </div>
        <div class="modal-backdrop" @click="emit('close')"></div>
    </div>
</template>
