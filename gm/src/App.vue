<script setup lang="ts">
import GameOverview from './gameOverview/GameOverview.vue';
import GmAuth from './auth/GmAuth.vue';
import ModalController from './modal/ModalController.vue';
import { useAuthStore } from './auth/authStore';

const authStore = useAuthStore();
</script>

<template>
    <GmAuth v-if="!authStore.isAuthenticated" />
    <GameOverview v-else />
    <ModalController />
</template>
