<script setup lang="ts">
import { useAuthStore } from './auth/authStore';
import UserAuth from './auth/UserAuth.vue';
import GameOverview from './gameOverview/GameOverview.vue';

const authStore = useAuthStore();
</script>

<template>
    <div class="container">
        <h1 class="display-1 text-center">Turnkeeper user client</h1>
        <UserAuth v-if="!authStore.isAuthenticated" />
        <GameOverview v-else />
    </div>
</template>
