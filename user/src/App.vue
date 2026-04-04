<script setup lang="ts">
import { onMounted } from 'vue';
import { useWsConnection } from './api/useWsConnection';
import { useAuthStore } from './auth/authStore';
import GamePage from './game/GamePage.vue';
import GameOverview from './gameOverview/GameOverview.vue';
import { useUsersStore } from './users/usersStore';
import AuthPage from './auth/AuthPage.vue';

const authStore = useAuthStore();
const wsConnection = useWsConnection();
const usersStore = useUsersStore();

onMounted(() => {
    wsConnection.autoConnect();
    usersStore.loadUsers();
});
</script>

<template>
    <div class="container">
        <AuthPage v-if="!authStore.isAuthenticated" />
        <GamePage v-else-if="wsConnection.isConnected.value" />
        <GameOverview v-else />
    </div>
</template>
