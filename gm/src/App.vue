<script setup lang="ts">
import GameOverview from './gameOverview/GameOverview.vue';
import GmAuth from './auth/GmAuth.vue';
import ModalController from './modal/ModalController.vue';
import { useAuthStore } from './auth/authStore';

const authStore = useAuthStore();
</script>

<template>
    <div class="container btn">
        <h1 class="display-1 text-center">Turnkeeper user client</h1>
        <GmAuth v-if="!authStore.isAuthenticated" />
        <GameOverview v-else />
    </div>
    <ModalController />
</template>
