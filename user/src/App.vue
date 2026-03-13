<script setup lang="ts">
import { useWsConnection } from './api/useWsConnection';
import { useAuthStore } from './auth/authStore';
import UserAuth from './auth/UserAuth.vue';
import GamePage from './game/GamePage.vue';
import GameOverview from './gameOverview/GameOverview.vue';
import { useUsersStore } from './users/usersStore';

const authStore = useAuthStore();
const wsConnection = useWsConnection();

// Load users on app startup
const usersStore = useUsersStore();
usersStore.loadUsers();
</script>

<template>
    <div class="container">
        <UserAuth v-if="!authStore.isAuthenticated" />
        <GamePage v-else-if="wsConnection.isConnected.value" />
        <GameOverview v-else />
    </div>
</template>
