<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useGameEditor } from './useGameEditor';

const route = useRoute();

const gameEditor = useGameEditor();

const load = () => {
    gameEditor.loadGame(route.params.id as string);
};
</script>

<template>
    <div v-if="gameEditor.game.value.status === 'uninitialized'">
        <h2 class="text-xl font-bold mb-4">Uninitialized</h2>
        <button class="btn btn-primary" @click="load">Load Game</button>
    </div>
    <div v-else-if="gameEditor.game.value.status === 'loading'">
        <h2 class="text-xl font-bold mb-4">Loading...</h2>
    </div>
    <div v-else-if="gameEditor.game.value.status === 'error'">
        <h2 class="text-xl font-bold mb-4">Error</h2>
        <p>{{ gameEditor.game.value.error }}</p>
        <button class="btn btn-primary" @click="load">Retry</button>
    </div>
    <div v-else-if="gameEditor.game.value.status === 'success'">
        <h2 class="text-2xl font-bold">{{ gameEditor.game.value.data.name }}</h2>
        <p class="text-sm mb-4">{{ gameEditor.game.value.data.description }}</p>
        <p class="text-sm">
            Created At:
            {{ new Date(gameEditor.game.value.data.created_at).toDateString() }}
        </p>
        <p class="text-sm">
            Updated At:
            {{ new Date(gameEditor.game.value.data.updated_at).toDateString() }}
        </p>
    </div>
    <RouterLink
        :to="{ name: 'game', params: { id: route.params.id } }"
        class="btn btn-secondary mt-4"
    >
        Back to Game View
    </RouterLink>
</template>
