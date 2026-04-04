<script setup lang="ts">
import GameOverview from './gameOverview/GameOverview.vue';
import ModalController from './common/modal/ModalController.vue';
import { useAuthStore } from './auth/authStore';
import { useWsConnection } from './api/useWsConnection';
import GamePage from './game/GamePage.vue';
import { useUsersStore } from './users/usersStore';
import { onMounted } from 'vue';
import AuthPage from './auth/AuthPage.vue';

const authStore = useAuthStore();
const wsConnection = useWsConnection();
const usersStore = useUsersStore();

onMounted(() => {
    usersStore.loadUsers();
    wsConnection.autoConnect();
});
</script>

<template>
    <AuthPage v-if="!authStore.isAuthenticated" />
    <GamePage v-else-if="wsConnection.isConnected.value" />
    <GameOverview v-else />
    <ModalController />
</template>
