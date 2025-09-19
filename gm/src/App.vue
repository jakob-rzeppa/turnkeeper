<script setup lang="ts">
import PlayerDisplay from './components/PlayerDisplay.vue'
import PlayerListener from './listeners/PlayerListener.vue'
import ModalController from './components/modal/ModalController.vue'
import ConnectionController from './components/ConnectionController.vue'
</script>

<template>
    <main>
        <ConnectionController />
        <PlayerDisplay />
        <PlayerListener />
    </main>
    <ModalController />
</template>
