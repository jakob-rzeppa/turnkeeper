<script setup lang="ts">
import connectionService, { socket } from './connectionHandlers/connection'

import Login from './components/Login.vue'
import PlayerDisplay from './components/PlayerDisplay.vue'

const isConnected = connectionService.isConnected()

function sendMessage() {
    socket.send('hi')
}

function disconnect() {
    connectionService.disconnect()
}
</script>

<template>
    <Login v-if="!isConnected" />
    <main>
        <h1>Hello</h1>
        <button @click="sendMessage">Send Message</button>
        <button @click="disconnect">Disconnect</button>
        <PlayerListener>
            <PlayerDisplay />
        </PlayerListener>
    </main>
</template>
