<script setup lang="ts">
import PlayerDisplay from './components/display/PlayerDisplay.vue'
import PlayerListener from './listeners/PlayerListener.vue'
import ModalController from './components/modal/ModalController.vue'
import ConnectionController from './components/ConnectionController.vue'
import TurnDisplay from './components/display/TurnDisplay.vue'
import TurnListener from './listeners/TurnListener.vue'
import LogDisplay from './components/display/LogDisplay.vue'
import LogListener from './listeners/LogListener.vue'
import Drawer from './components/Drawer.vue'
import InitGameButton from './components/InitGameButton.vue'
import connection from './composables/connection'
import CreatePlayerButton from './components/CreatePlayerButton.vue'
import UpdatePlayerOrderButton from './components/UpdatePlayerOrderButton.vue'

const { isConnected } = connection()
</script>

<template>
    <main class="bg-base-300 h-screen p-12">
        <Drawer>
            <ConnectionController />
            <InitGameButton />
            <UpdatePlayerOrderButton />
            <CreatePlayerButton />
        </Drawer>
        <div v-if="isConnected">
            <div class="grid grid-cols-2 gap-4">
                <TurnDisplay />
                <LogDisplay />
                <PlayerDisplay />
            </div>
            <TurnListener />
            <LogListener />
            <PlayerListener />
        </div>
        <div v-else class="flex flex-col items-center justify-center h-full">
            <span class="loading loading-bars loading-xl"></span>
        </div>
    </main>
    <ModalController />
</template>
