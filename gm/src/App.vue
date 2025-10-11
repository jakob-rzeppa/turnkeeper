<script setup lang="ts">
import PlayerDisplay from './components/player/PlayerDisplay.vue'
import ModalController from './components/modal/ModalController.vue'
import ConnectionController from './components/connection/ConnectionController.vue'
import TurnDisplay from './components/display/TurnDisplay.vue'
import LogDisplay from './components/log/LogDisplay.vue'
import Drawer from './components/container/DrawerContainer.vue'
import InitGameButton from './components/game/InitGameButton.vue'
import connection from './composables/connection'
import CreatePlayerButton from './components/player/CreatePlayerButton.vue'
import UpdatePlayerOrderButton from './components/game/UpdatePlayerOrderButton.vue'
import EndGameButton from './components/game/EndGameButton.vue'
import { useGameStore } from './stores/gameStore'
import { useLogStore } from './stores/logStore'
import { usePlayerStore } from './stores/playerStore'

const { isConnected } = connection()

// Initialize all stores, that define listeners for backend updates, so that they are ready when the gm connects to the backend
useGameStore()
useLogStore()
usePlayerStore()
</script>

<template>
    <main class="bg-base-300 h-screen p-12">
        <Drawer>
            <ConnectionController />
            <InitGameButton />
            <UpdatePlayerOrderButton />
            <CreatePlayerButton />
            <EndGameButton />
        </Drawer>
        <div v-if="isConnected">
            <div class="grid grid-cols-2 gap-4">
                <TurnDisplay />
                <LogDisplay />
                <PlayerDisplay />
            </div>
        </div>
        <div v-else class="flex flex-col items-center justify-center h-full">
            <span class="loading loading-bars loading-xl"></span>
        </div>
    </main>
    <ModalController />
</template>
