<script setup lang="ts">
import { onMounted, ref } from 'vue';
import type { GameDetails } from '../types/game';
import { getGameDetails } from '../api/requests/games/getGameDetails';
import { getGameInstances } from '../api/requests/gameInstances/getGameInstances';
import { useRoute } from 'vue-router';
import type { GameInstanceMetadata } from '../types/gameInstances';
import { useModalStore } from '../common/modal/modalStore';
import CreateGameInstanceModal from '../gameInstances/CreateGameInstanceModal.vue';
import DeleteGameInstanceModal from '../gameInstances/DeleteGameInstanceModal.vue';
import type { DataState } from '../types/util';

const route = useRoute();
const modalStore = useModalStore();

const game = ref<DataState<GameDetails>>({ status: 'loading' });
const gameInstances = ref<DataState<GameInstanceMetadata[]>>({ status: 'loading' });

const loadGameInstances = async () => {
    const gameInstancesResponse = await getGameInstances(route.params.id as string);

    if (gameInstancesResponse.isOk()) {
        gameInstances.value = {
            status: 'success',
            data: gameInstancesResponse.value,
        };
    } else {
        gameInstances.value = { status: 'error', error: gameInstancesResponse.error };
    }
};

const loadGame = async () => {
    game.value = { status: 'loading' };
    gameInstances.value = { status: 'loading' };
    const gameResponse = await getGameDetails(route.params.id as string);

    if (gameResponse.isOk()) {
        game.value = { status: 'success', data: gameResponse.value };
    } else {
        game.value = { status: 'error', error: gameResponse.error };
        gameInstances.value = { status: 'error', error: 'Failed to load game.' };
        return;
    }

    loadGameInstances();
};

onMounted(() => {
    loadGame();
});
</script>

<template>
    <div class="min-h-screen bg-base-200 p-8">
        <div class="max-w-6xl mx-auto">
            <div v-if="game.status === 'loading'" class="flex justify-center">
                <span class="loading loading-spinner loading-lg text-primary"></span>
            </div>

            <div v-else-if="game.status === 'error'" class="alert alert-error shadow-lg">
                <span>{{ game.error }}</span>
            </div>

            <div v-else>
                <h1 class="text-4xl font-bold text-base-content mb-4">{{ game.data.name }}</h1>
                <p class="text-base-content/80 mb-4">{{ game.data.description }}</p>
                <div class="text-sm text-base-content/60 space-y-1">
                    <p>
                        <span class="font-semibold">Created:</span>
                        {{ new Date(game.data.created_at).toLocaleDateString() }}
                    </p>
                    <p>
                        <span class="font-semibold">Updated:</span>
                        {{ new Date(game.data.updated_at).toLocaleDateString() }}
                    </p>
                </div>
                <RouterLink
                    :to="{ name: 'game-editor', params: { id: route.params.id } }"
                    class="btn btn-link btn-accent mt-4"
                    >Editor</RouterLink
                >
                <pre class="bg-base-100 p-4 rounded-lg mt-6 overflow-x-auto">
                    {{ game.data.source_code }}
                </pre>

                <div class="divider"></div>

                <h2 class="text-2xl font-bold text-base-content mb-4">Game Instances</h2>

                <div v-if="gameInstances.status === 'loading'" class="flex justify-center">
                    <span class="loading loading-spinner loading-lg text-primary"></span>
                </div>

                <div
                    v-else-if="gameInstances.status === 'error'"
                    class="alert alert-error shadow-lg"
                >
                    <span>{{ gameInstances.error }}</span>
                </div>

                <div
                    v-else-if="
                        gameInstances.status === 'success' && gameInstances.data.length === 0
                    "
                    class="alert alert-info shadow-lg"
                >
                    <span>No game instances found.</span>
                </div>

                <div v-else class="space-y-4">
                    <div
                        v-for="instance in gameInstances.data"
                        :key="instance.id"
                        class="card bg-base-100 shadow-md"
                    >
                        <div class="card-body">
                            <h3 class="card-title">{{ instance.name }}</h3>
                            <p class="text-sm text-base-content/60">
                                Created: {{ new Date(instance.created_at).toLocaleDateString() }}
                            </p>
                            <button
                                class="btn btn-error w-max"
                                @click="
                                    modalStore.openModal(
                                        DeleteGameInstanceModal,
                                        {
                                            gameId: route.params.id,
                                            gameInstanceId: instance.id,
                                            gameInstanceName: instance.name,
                                        },
                                        { deleted: () => loadGameInstances() }
                                    )
                                "
                            >
                                Delete
                            </button>
                        </div>
                    </div>
                </div>
                <button
                    class="btn btn-primary mt-6"
                    @click="
                        modalStore.openModal(
                            CreateGameInstanceModal,
                            {
                                gameId: route.params.id,
                                gameName: game.data.name || '',
                            },
                            { created: () => loadGameInstances() }
                        )
                    "
                >
                    Create New Game Instance
                </button>
            </div>
        </div>
    </div>
</template>
