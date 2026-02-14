<script setup lang="ts">
import GameOverview from './gameOverview/GameOverview.vue';
import GmAuth from './auth/GmAuth.vue';
import ModalController from './modal/ModalController.vue';
import { useAuthStore } from './auth/authStore';
import { useWsConnection } from './api/useWsConnection';
import GamePage from './game/GamePage.vue';

const authStore = useAuthStore();
const wsConnection = useWsConnection();
</script>

<template>
    <GmAuth v-if="!authStore.isAuthenticated" />
    <GamePage v-else-if="wsConnection.isConnected.value" />
    <GameOverview v-else />
    <ModalController />
</template>
