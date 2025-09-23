<script setup lang="ts">
import { useLogStore } from '@/stores/logStore'
import { socket } from '../util/connection'

const logStore = useLogStore()

socket.on('log', (logEntry) => {
    logStore.logs.push(logEntry)
})
</script>

<template></template>
