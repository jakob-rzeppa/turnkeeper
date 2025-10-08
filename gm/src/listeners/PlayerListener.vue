<script setup lang="ts">
import useConnection from '@/composables/connection'
import { usePlayerStore } from '@/stores/playerStore'

const playerStore = usePlayerStore()
const { socket } = useConnection()

socket.on('players', (players) => {
    playerStore.players = players
})
</script>

<template></template>
