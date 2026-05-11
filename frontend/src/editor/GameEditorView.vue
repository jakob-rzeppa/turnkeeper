<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import type { GameDetails } from '../types/game';
import { getGameDetails } from '../api/requests/games/getGameDetails';
import { updateSourceCode } from '../api/requests/games/updateSourceCode';
import { checkGame } from '../api/requests/games/checkGame';
import type { DataState } from '../types/util';
import type { CheckGamesResponse } from '../api/requests/games/checkGame';

const route = useRoute();

const game = ref<DataState<GameDetails>>({ status: 'loading' });
const checkResult = ref<DataState<CheckGamesResponse> | null>(null);

const sourceCode = ref<string>('');

const isSaved = computed(() => {
    if (game.value.status !== 'success') return false;
    return sourceCode.value === game.value.data.source_code;
});

const loadGame = async () => {
    game.value = { status: 'loading' };
    const res = await getGameDetails(route.params.id as string);

    if (res.isOk()) {
        game.value = { status: 'success', data: res.value };
        sourceCode.value = res.value.source_code;
    } else {
        game.value = { status: 'error', error: res.error };
    }
};

const handleUpdateSourceCode = async (newSourceCode: string) => {
    if (game.value.status === 'loading' || game.value.status === 'error') return;
    game.value = { status: 'loading' };

    const res = await updateSourceCode(route.params.id as string, newSourceCode);

    if (res.isOk()) {
        loadGame();
    } else {
        game.value = { status: 'error', error: res.error };
    }
};

const checkSourceCode = async () => {
    checkResult.value = { status: 'loading' };
    const res = await checkGame(route.params.id as string);

    if (res.isOk()) {
        checkResult.value = { status: 'success', data: res.value };
    } else {
        checkResult.value = { status: 'error', error: res.error };
    }
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
            <button @click="handleUpdateSourceCode(sourceCode)" class="btn btn-primary">
                Update Source Code
            </button>
            <button :disabled="!isSaved" @click="checkSourceCode" class="btn btn-secondary ml-2">
                Check Source Code
            </button>

            <div v-if="checkResult" class="mt-8">
                <div v-if="checkResult.status === 'loading'" class="flex justify-center">
                    <span class="loading loading-spinner loading-lg text-primary"></span>
                </div>

                <div v-else-if="checkResult.status === 'error'" class="alert alert-error shadow-lg">
                    <span>{{ checkResult.error }}</span>
                </div>

                <div
                    v-else-if="checkResult.data.is_valid === false"
                    class="alert alert-error shadow-lg"
                >
                    <div>
                        <p class="font-semibold mb-2">Validation Errors:</p>
                        <ul class="list-disc list-inside">
                            <li v-for="(error, index) in checkResult.data.errors" :key="index">
                                {{ error }}
                            </li>
                        </ul>
                    </div>
                </div>

                <div v-else class="alert alert-success shadow-lg">
                    <div class="max-w-full">
                        <p class="font-semibold mb-2">Source Code is Valid!</p>
                        <details class="collapse bg-base-300">
                            <summary class="collapse-title font-semibold">Output Details</summary>
                            <div class="collapse-content">
                                <pre class="bg-base-100 p-4 rounded overflow-auto text-xs">{{
                                    JSON.stringify(checkResult.data.output, null, 2)
                                }}</pre>
                            </div>
                        </details>
                    </div>
                </div>
            </div>
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
