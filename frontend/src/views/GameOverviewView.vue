<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useModalStore } from '../common/modal/modalStore';
import CreateGamesModal from '../games/CreateGameModal.vue';
import DeleteGameModal from '../games/DeleteGameModal.vue';
import { type DataState } from '../types/util';
import type { GameMetadata } from '../types/game';
import { getGames } from '../api/requests/games/getGames';

const modalStore = useModalStore();

const games = ref<DataState<GameMetadata[]>>({ status: 'loading' });

const loadGames = async () => {
    games.value = { status: 'loading' };

    const res = await getGames();

    if (res.isOk()) {
        games.value = { status: 'success', data: res.value };
    } else {
        games.value = { status: 'error', error: res.error };
    }
};

onMounted(() => {
    loadGames();
});
</script>

<template>
    <div class="min-h-screen bg-base-200 p-8">
        <div class="max-w-6xl mx-auto">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-4xl font-bold text-base-content">Games</h1>
                <button
                    @click="
                        () =>
                            modalStore.openModal(
                                CreateGamesModal,
                                {},
                                {
                                    create: loadGames,
                                }
                            )
                    "
                    class="btn btn-primary"
                >
                    Create Game
                </button>
            </div>

            <div v-if="games.status === 'loading'" class="flex justify-center">
                <span class="loading loading-spinner loading-lg text-primary"></span>
            </div>

            <div v-else-if="games.status === 'error'" class="alert alert-error shadow-lg">
                <span>{{ games.error }}</span>
            </div>

            <div v-else>
                <div v-if="games.data.length === 0" class="text-center py-12">
                    <p class="text-xl text-base-content/60">
                        No games yet. Create one to get started!
                    </p>
                </div>
                <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div
                        v-for="game in games.data"
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
                                <RouterLink
                                    :to="{ name: 'game', params: { id: game.id } }"
                                    class="btn btn-sm btn-outline"
                                    >View</RouterLink
                                >
                                <button
                                    @click="
                                        modalStore.openModal(
                                            DeleteGameModal,
                                            {
                                                gameId: game.id,
                                                gameName: game.name,
                                            },
                                            {
                                                delete: loadGames,
                                            }
                                        )
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
