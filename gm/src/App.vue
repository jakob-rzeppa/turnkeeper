<script setup lang="ts">
import GameOverview from './components/GameOverview.vue';
import GmAuth from './components/GmAuth.vue';
import { useAuthStore } from './stores/auth';

const authStore = useAuthStore();
</script>

<template>
    <div class="container">
        <h1 class="display-1 text-center">Turnkeeper user client</h1>
        <GmAuth v-if="!authStore.isAuthenticated" />
        <GameOverview v-else />
    </div>
</template>
