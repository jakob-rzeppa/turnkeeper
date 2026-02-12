<script setup lang="ts">
import { ref } from 'vue';
import type { GameMetadata } from '../types/game';
import { request } from '../api/httpApi';
import { useModalStore } from '../modal/modalStore';
import CreateGameModal from './CreateGameModal.vue';

const modalStore = useModalStore();

const response = ref(request<{ games: GameMetadata[] }>('GET', '/games'));

function openCreateGameModal() {
    modalStore.openModal(CreateGameModal);
}
</script>

<template>
    <div v-if="response.loading" class="flex items-center justify-center h-48">
        <span class="loading loading-spinner loading-lg" aria-hidden="true"></span>
    </div>

    <div v-else-if="response.error" class="flex items-center justify-center h-48">
        <div class="alert alert-error">
            <strong>Error!</strong>
            <div>Failed to load games: {{ response.error.message || 'Unknown error' }}</div>
        </div>
    </div>

    <div v-else class="container mx-auto p-4 space-y-6">
        <h2 class="text-4xl font-semibold text-center">Games Overview</h2>

        <div v-if="response.payload.games.length === 0" class="text-center">
            No games available.
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div
                v-for="game in response.payload.games"
                :key="game.id"
                class="card bg-base-200 shadow-md hover:shadow-xl transition-colors h-full"
            >
                <div class="card-body flex flex-col gap-3">
                    <h3 class="card-title text-primary">{{ game.name }}</h3>

                    <div class="space-x-2">
                        <span class="badge badge-secondary"
                            >Players: {{ game.number_of_players }}</span
                        >
                        <span class="badge badge-ghost">Round: {{ game.round_number }}</span>
                    </div>

                    <div class="card-actions mt-auto">
                        <button class="btn btn-outline btn-primary btn-sm w-full">Resume</button>
                    </div>
                </div>
            </div>
        </div>
        <button @click="openCreateGameModal" class="btn btn-primary btn-block w-full">
            Create New Game
        </button>
    </div>
</template>
