<script setup lang="ts">
import connectionService from '@/util/connection'
</script>

<template>
    <button @click="() => connectionService.connect()">Login</button>
</template>
