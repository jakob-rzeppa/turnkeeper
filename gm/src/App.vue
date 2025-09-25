<script setup lang="ts">
import PlayerDisplay from './components/PlayerDisplay.vue'
import PlayerListener from './listeners/PlayerListener.vue'
import ModalController from './components/modal/ModalController.vue'
import ConnectionController from './components/ConnectionController.vue'
import TurnDisplay from './components/TurnDisplay.vue'
import TurnListener from './listeners/TurnListener.vue'
import LogDisplay from './components/LogDisplay.vue'
import LogListener from './listeners/LogListener.vue'
</script>

<template>
    <main class="bg-base-300 h-screen p-12">
        <ConnectionController />
        <div class="grid">
            <TurnDisplay />
            <LogDisplay />
            <PlayerDisplay />
        </div>
        <TurnListener />
        <LogListener />
        <PlayerListener />
    </main>
    <ModalController />
</template>
