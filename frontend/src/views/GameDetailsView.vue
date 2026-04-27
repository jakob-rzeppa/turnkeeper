<script setup lang="ts">
import { onMounted, ref } from 'vue';
import type { GameDetails } from '../types/game';
import { getWithAuth } from '../api/httpApi';
import { useRoute } from 'vue-router';
import type { GameInstanceMetadata } from '../types/gameInstances';
import { useModalStore } from '../common/modal/modalStore';
import CreateGameInstanceModal from '../gameInstances/CreateGameInstanceModal.vue';
import DeleteGameInstanceModal from '../gameInstances/DeleteGameInstanceModal.vue';

const route = useRoute();
const modalStore = useModalStore();

const game = ref<
    | { loading: true }
    | { loading: false; error: string }
    | { loading: false; error: null; data: GameDetails }
>({ loading: true });
const gameInstances = ref<
    | { loading: true }
    | { loading: false; error: string }
    | { loading: false; error: null; data: GameInstanceMetadata[] }
>({ loading: true });

const loadGameInstances = async () => {
    const gameInstancesResponse = await getWithAuth<{ game_instances: GameInstanceMetadata[] }>(
        `/games/${route.params.id}/instances`
    );

    if (gameInstancesResponse.isOk()) {
        gameInstances.value = {
            loading: false,
            error: null,
            data: gameInstancesResponse.value.game_instances,
        };
    } else {
        gameInstances.value = { loading: false, error: gameInstancesResponse.error.message };
    }
};

const loadGame = async () => {
    game.value = { loading: true };
    gameInstances.value = { loading: true };
    const gameResponse = await getWithAuth<GameDetails>(`/games/${route.params.id}`);

    if (gameResponse.isOk()) {
        game.value = { loading: false, error: null, data: gameResponse.value };
    } else {
        game.value = { loading: false, error: gameResponse.error.message };
        gameInstances.value = { loading: false, error: 'Failed to load game.' };
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
            <div v-if="game.loading === true" class="flex justify-center">
                <span class="loading loading-spinner loading-lg text-primary"></span>
            </div>

            <div v-else-if="game.error !== null" class="alert alert-error shadow-lg">
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

                <div v-if="gameInstances.loading === true" class="flex justify-center">
                    <span class="loading loading-spinner loading-lg text-primary"></span>
                </div>

                <div v-else-if="gameInstances.error !== null" class="alert alert-error shadow-lg">
                    <span>{{ gameInstances.error }}</span>
                </div>

                <div v-else-if="gameInstances.data.length === 0" class="alert alert-info shadow-lg">
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
