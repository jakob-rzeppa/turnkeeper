<script setup lang="ts">
import Login from './components/Login.vue'
import useConnection from './composables/connection'

const { isConnected } = useConnection()
</script>

<template>
    <Login v-if="!isConnected" />
</template>

<style scoped></style>
