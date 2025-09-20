<script setup lang="ts">
import { socket } from '@/util/connection'
import { onMounted, ref } from 'vue'

const playerOrderRef = ref<{ id: string; name: string }[]>([])

socket.on('gameloop:order', (data: { playerOrder: { id: string; name: string }[] }) => {
    playerOrderRef.value = data.playerOrder
})

const initGameLoop = () => {
    socket.emit('gameloop:init', {})
}
</script>

<template>
    <div>
        <button class="btn btn-secondary btn-sm mb-2" @click="initGameLoop">Init Game Loop</button>
        <div class="breadcrumbs">
            <ul>
                <li v-for="player in playerOrderRef" :key="player.id">{{ player.name }}</li>
            </ul>
        </div>
    </div>
</template>
