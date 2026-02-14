<script setup lang="ts">
import { useWsConnection } from '../api/useWsConnection';

const wsConnection = useWsConnection();
</script>

<template>
    <div>
        <h1>Game Page</h1>
        <p>This is the game page.</p>
        <button @click="wsConnection.disconnect" class="btn btn-outline btn-secondary">
            Disconnect
        </button>
    </div>
</template>
