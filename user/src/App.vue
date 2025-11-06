<script setup lang="ts">
import GameDisplay from './components/GameDisplay.vue';
import LoginForm from './components/LoginForm.vue';
import LogoutButton from './components/LogoutButton.vue';
import MainContainer from './components/MainContainer.vue';
import PlayerNameDisplay from './components/PlayerNameDisplay.vue';
import PlayerNotesDisplay from './components/PlayerNotesDisplay.vue';
import PlayerDisplay from './components/StatsDisplay.vue';
import useConnection from './composables/useConnection';
import { useGameStateStore } from './stores/gameStateStore';
import { useMessagesStore } from './stores/messagesStore';
import { usePlayerStore } from './stores/playerStore';

const { isConnected } = useConnection();

// Initialize all stores, that define listeners for backend updates, so that they are ready when the user connects to the backend
usePlayerStore();
useGameStateStore();
useMessagesStore();
</script>

<template>
    <div class="min-h-screen">
        <LoginForm v-if="!isConnected" />
        <MainContainer v-else>
            <PlayerNameDisplay />
            <GameDisplay />
            <PlayerNotesDisplay />
            <PlayerDisplay />
        </MainContainer>
        <LogoutButton v-if="isConnected" />
        <!-- Footer spacer for mobile (to scroll down) -->
        <div class="h-[50vh]"></div>
    </div>
</template>
