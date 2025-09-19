<script setup lang="ts">
import connectionService, { socket } from './util/connection'

import Login from './components/Login.vue'
import PlayerDisplay from './components/PlayerDisplay.vue'
import PlayerListener from './listeners/PlayerListener.vue'
import ModalController from './components/modal/ModalController.vue'

const isConnected = connectionService.isConnected()

function disconnect() {
    connectionService.disconnect()
}
</script>

<template>
    <Login v-if="!isConnected" />
    <main>
        <button @click="disconnect">Disconnect</button>
        <PlayerDisplay />
        <PlayerListener />
    </main>
    <ModalController />
</template>
