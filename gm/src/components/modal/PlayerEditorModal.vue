<script setup lang="ts">
import type { Player } from '@/types/player'
import { socket } from '@/util/connection'
import { ref, shallowRef } from 'vue'
import NewStatModal from './NewStatModal.vue'
import { useModalStore } from '@/stores/modalStore'

const props = defineProps<{
    player: Player
}>()

const emit = defineEmits(['close'])

const modalStore = useModalStore()

const playerName = ref(props.player.name)
const playerSecret = ref(props.player.secret)
const playerStats = ref(props.player.stats)

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

function openNewStatModal() {
    const newStatModal = shallowRef(NewStatModal)
    modalStore.openModal(newStatModal, { playerId: props.player.id, playerName: props.player.name })
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
    <div class="flex flex-col gap-2">
        <h2 class="text-2xl text-center text-primary">Stats</h2>
        <label v-for="stat in playerStats" :key="stat.name">
            <span class="label">{{ stat.name }}</span>
            <input type="text" v-model="stat.value" />
        </label>
        <button class="btn btn-secondary" @click="openNewStatModal">Add Stat</button>
    </div>
    <button class="btn btn-primary" @click="updatePlayer">Update Player</button>
</template>
