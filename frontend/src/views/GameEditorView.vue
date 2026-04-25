<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import type { GameDetails } from '../types/game';
import { getWithAuth, patchWithAuth } from '../api/httpApi';

const route = useRoute();

const game = ref<
    | { loading: true }
    | { loading: false; error: string }
    | { loading: false; error: null; data: GameDetails }
>({ loading: true });

const sourceCode = ref<string>('');

const loadGame = async () => {
    game.value = { loading: true };
    const res = await getWithAuth<GameDetails>(`/games/${route.params.id}`);

    if (res.isOk()) {
        game.value = { loading: false, error: null, data: res.value };
        sourceCode.value = res.value.source_code;
    } else {
        game.value = { loading: false, error: res.error.message };
    }
};

const updateSourceCode = async (newSourceCode: string) => {
    if (game.value.loading === true || game.value.error !== null) return;
    game.value = { loading: true };

    const res = await patchWithAuth(`/games/${route.params.id}/source-code`, {
        source_code: newSourceCode,
    });

    if (res.isOk()) {
        loadGame();
    } else {
        game.value = { loading: false, error: res.error.message };
    }
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
                <div class="text-sm text-base-content/60 space-y-1 mb-4">
                    <p>
                        <span class="font-semibold">Created:</span>
                        {{ new Date(game.data.created_at).toLocaleDateString() }}
                    </p>
                    <p>
                        <span class="font-semibold">Updated:</span>
                        {{ new Date(game.data.updated_at).toLocaleDateString() }}
                    </p>
                </div>
            </div>

            <label for="source-code" class="block text-sm font-medium text-base-content mb-2"
                >Source Code</label
            >
            <textarea
                id="source-code"
                v-model="sourceCode"
                rows="10"
                class="textarea textarea-bordered w-full mb-4"
            ></textarea>
            <button @click="updateSourceCode(sourceCode)" class="btn btn-primary">
                Update Source Code
            </button>
        </div>
        <div class="flex flex-row justify-center">
            <RouterLink :to="{ name: 'game-overview' }" class="btn btn-link btn-warning"
                >Back to Overview</RouterLink
            >
            <RouterLink
                :to="{ name: 'game', params: { id: route.params.id } }"
                class="btn btn-link btn-warning"
                >Back to Game Details</RouterLink
            >
        </div>
    </div>
</template>
