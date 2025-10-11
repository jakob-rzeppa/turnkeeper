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
    <main class="bg-base-300 min-h-screen">
        <Drawer>
            <ConnectionController />
            <InitGameButton />
            <UpdatePlayerOrderButton />
            <CreatePlayerButton />
            <EndGameButton />
        </Drawer>
        <div class="container mx-auto p-6 lg:p-8">
            <div v-if="isConnected" class="space-y-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
                    <div class="lg:col-span-1">
                        <TurnDisplay />
                    </div>
                    <div class="lg:col-span-1">
                        <LogDisplay />
                    </div>
                    <div class="lg:col-span-2 xl:col-span-3">
                        <PlayerDisplay />
                    </div>
                </div>
            </div>
            <div v-else class="flex flex-col items-center justify-center min-h-[70vh] space-y-4">
                <div class="text-center">
                    <span class="loading loading-spinner loading-lg text-primary"></span>
                    <h2 class="text-2xl font-semibold text-base-content mt-4">
                        Not connected to Server
                    </h2>
                </div>
            </div>
        </div>
    </main>
    <ModalController />
</template>
