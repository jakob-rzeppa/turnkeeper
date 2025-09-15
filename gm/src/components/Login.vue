<script setup lang="ts">
import connectionService from '@/connectionHandlers/connection'
</script>

<template>
    <button @click="() => connectionService.connect()">Login</button>
</template>
