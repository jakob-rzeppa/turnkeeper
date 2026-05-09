<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useSessionConnection } from './useSessionConnection';
import { RouterLink, useRoute } from 'vue-router';

const route = useRoute();

const sessionConnection = useSessionConnection();

const connectToSession = () => {
    sessionConnection.connect(route.params.gameId as string, route.params.gameInstanceId as string);
};

const disconnectFromSession = () => {
    if (sessionConnection.connectionStatus.value === 'connected') {
        sessionConnection.disconnect();
    }
};

onMounted(() => {
    connectToSession();
})

onUnmounted(() => {
    disconnectFromSession();
});
</script>

<template>
    <div>
        <h1>Game Session View</h1>
        
        <RouterLink :to="{ name: 'game', params: { id: route.params.gameId } }" class="btn btn-secondary mb-4">
            Back to Game Details
        </RouterLink>

        <div v-if="sessionConnection.connectionStatus.value === 'disconnected'" class="alert alert-warning shadow-lg">
            <span>Not connected to Game Session.</span>
            <button class="btn btn-sm btn-primary" @click="connectToSession">Connect</button>
        </div>
        <div v-else-if="sessionConnection.connectionStatus.value === 'connecting'" class="alert alert-info shadow-lg">
            <span>Connecting to Game Session...</span>
        </div>
        <div v-else-if="sessionConnection.connectionStatus.value === 'error'" class="alert alert-error shadow-lg">
            <span>Failed to connect to Game Session.</span>
            <button class="btn btn-sm btn-primary" @click="connectToSession">Retry</button>
        </div>
        <div v-else-if="sessionConnection.connectionStatus.value === 'connected'" class="alert alert-success shadow-lg">
            <span>Connected to Game Session!</span>
            <div v-if="sessionConnection.displayTemplate">
                <h2 class="mt-4">Display Template:</h2>
                <pre>{{ sessionConnection.displayTemplate }}</pre>
            </div>
            <div v-if="sessionConnection.gameState">
                <h2 class="mt-4">Game State:</h2>
                <pre>{{ sessionConnection.gameState }}</pre>
            </div>
            <button class="btn btn-sm btn-error" @click="disconnectFromSession">Disconnect</button>
        </div>
    </div>
</template>