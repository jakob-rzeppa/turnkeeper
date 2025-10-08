<script setup lang="ts">
import useConnection from '@/composables/connection'
import { useLogStore } from '@/stores/logStore'

const logStore = useLogStore()
const { socket } = useConnection()

socket.on('log', (logEntry) => {
    logStore.logs.push(logEntry)
})
</script>

<template></template>
