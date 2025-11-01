<script setup lang="ts">
import PlayerDisplay from './components/player/PlayerDisplay.vue';
import ModalController from './components/modal/ModalController.vue';
import ConnectionController from './components/connection/ConnectionController.vue';
import TurnDisplay from './components/game/TurnDisplay.vue';
import CurrentPlayerDisplay from './components/game/CurrentPlayerDisplay.vue';
import LogDisplay from './components/log/LogDisplay.vue';
import Drawer from './components/container/DrawerContainer.vue';
import InitGameButton from './components/game/InitGameButton.vue';
import connection from './composables/useConnection';
import CreatePlayerButton from './components/player/CreatePlayerButton.vue';
import UpdatePlayerOrderButton from './components/game/UpdatePlayerOrderButton.vue';
import EndGameButton from './components/game/EndGameButton.vue';
import { useGameStore } from './stores/gameStore';
import { useLogStore } from './stores/logStore';
import { usePlayerStore } from './stores/playerStore';
import QuickActions from './components/QuickActions.vue';
import GameNotesEditor from './components/game/GameNotesEditor.vue';

const { isConnected } = connection();

// Initialize all stores, that define listeners for backend updates, so that they are ready when the gm connects to the backend
useGameStore();
useLogStore();
usePlayerStore();
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
        <div class="container mx-auto p-3">
            <div v-if="isConnected">
                <div class="grid grid-cols-5 gap-3">
                    <div class="col-span-2 row-span-1">
                        <TurnDisplay />
                    </div>
                    <div class="col-span-3 row-span-3">
                        <CurrentPlayerDisplay />
                    </div>
                    <div class="col-span-2 row-span-2">
                        <GameNotesEditor />
                    </div>
                    <div class="col-span-2">
                        <PlayerDisplay />
                    </div>
                    <div class="col-span-2">
                        <LogDisplay />
                    </div>
                    <div class="col-span-1">
                        <QuickActions />
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
