<script setup lang="ts">
import { onMounted, ref } from 'vue';
import type { GameDetails } from '../types/game';
import { getWithAuth } from '../api/httpApi';
import { useRoute } from 'vue-router';

const route = useRoute();

const game = ref<
    | { loading: true }
    | { loading: false; error: string }
    | { loading: false; error: null; data: GameDetails }
>({ loading: true });

const loadGame = async () => {
    game.value = { loading: true };
    const res = await getWithAuth<GameDetails>(`/games/${route.params.id}`);

    if (res.isOk()) {
        game.value = { loading: false, error: null, data: res.value };
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
                <pre class="bg-base-100 p-4 rounded-lg mt-6 overflow-x-auto">
                    {{ game.data.source_code }}
                </pre>
            </div>
        </div>
    </div>
</template>
