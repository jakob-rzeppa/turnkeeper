<script setup lang="ts">
import { socket } from '@/util/connection'
import { ref } from 'vue'

const props = defineProps<{
    player: { id: string; name: string; secret: string; stats: string[] }
}>()

const emit = defineEmits(['close'])

const playerName = ref(props.player.name)
const playerSecret = ref(props.player.secret)

const updatePlayer = () => {
    socket.emit('players:update', {
        playerId: props.player.id,
        playerData: {
            name: playerName.value,
            secret: playerSecret.value,
        },
    })
    emit('close')
}
</script>

<template>
    <h1 class="text-4xl text-center text-primary">Edit player</h1>
    <label class="input input-primary">
        <span class="label">Name</span>
        <input type="text" v-model="playerName" />
    </label>
    <label class="input input-primary">
        <span class="label">Secret</span>
        <input type="text" v-model="playerSecret" />
    </label>
    <button class="btn btn-primary" @click="updatePlayer">Update Player</button>
</template>
