<script setup lang="ts">
import ModalController from './common/modal/ModalController.vue';
import { useAuthStore } from './auth/authStore';
import { useWsConnection } from './api/useWsConnection';
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
    <RouterView v-else />
    <ModalController />
</template>
