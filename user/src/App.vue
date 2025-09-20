<script setup lang="ts">
import connectionService, { socket } from './util/connection'

import Login from './components/Login.vue'

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
  </main>
</template>

<style scoped></style>
