<script setup lang="ts">
import { useWsConnection } from './api/useWsConnection';
import { useAuthStore } from './auth/authStore';
import UserAuth from './auth/UserAuth.vue';
import GamePage from './game/GamePage.vue';
import GameOverview from './gameOverview/GameOverview.vue';

const authStore = useAuthStore();
const wsConnection = useWsConnection();
</script>

<template>
    <div class="container">
        <h1 class="display-1 text-center">Turnkeeper user client</h1>
        <UserAuth v-if="!authStore.isAuthenticated" />
        <GamePage v-else-if="wsConnection.isConnected.value" />
        <GameOverview v-else />
    </div>
</template>
