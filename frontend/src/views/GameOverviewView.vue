<script setup lang="ts">
import { useModalStore } from '../common/modal/modalStore';
import CreateGamesModal from '../games/CreateGameModal.vue';
import DeleteGameModal from '../games/DeleteGameModal.vue';
import { useGamesStore } from '../games/useGamesStore';

const modalStore = useModalStore();
const gamesStore = useGamesStore();
</script>

<template>
    <div class="min-h-screen bg-base-200 p-8">
        <div class="max-w-6xl mx-auto">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-4xl font-bold text-base-content">Games</h1>
                <button
                    @click="() => modalStore.openModal(CreateGamesModal)"
                    class="btn btn-primary"
                >
                    Create Game
                </button>
            </div>

            <div v-if="gamesStore.data.loading === true" class="flex justify-center">
                <span class="loading loading-spinner loading-lg text-primary"></span>
            </div>

            <div v-else-if="gamesStore.data.error !== null" class="alert alert-error shadow-lg">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-current shrink-0 h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M10 14l-2-2m0 0l-2-2m2 2l2-2m-2 2l-2 2m2-2l2 2m-4-4a7 7 0 1014 0 7 7 0 00-14 0z"
                    />
                </svg>
                <span>{{ gamesStore.data.error }}</span>
            </div>

            <div v-else>
                <div v-if="gamesStore.data.games.length === 0" class="text-center py-12">
                    <p class="text-xl text-base-content/60">
                        No games yet. Create one to get started!
                    </p>
                </div>
                <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div
                        v-for="game in gamesStore.data.games"
                        :key="game.id"
                        class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow"
                    >
                        <div class="card-body">
                            <h2 class="card-title text-2xl mb-2">{{ game.name }}</h2>
                            <p class="text-base-content/80 grow">{{ game.description }}</p>
                            <div class="divider my-2"></div>
                            <div class="text-sm text-base-content/60 space-y-1">
                                <p>
                                    <span class="font-semibold">Created:</span>
                                    {{ new Date(game.created_at).toLocaleDateString() }}
                                </p>
                                <p>
                                    <span class="font-semibold">Updated:</span>
                                    {{ new Date(game.updated_at).toLocaleDateString() }}
                                </p>
                            </div>
                            <div class="card-actions justify-end mt-4">
                                <button class="btn btn-sm btn-outline">View</button>
                                <button
                                    @click="
                                        modalStore.openModal(DeleteGameModal, {
                                            gameId: game.id,
                                            gameName: game.name,
                                        })
                                    "
                                    class="btn btn-sm btn-outline btn-error"
                                >
                                    Delete
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
