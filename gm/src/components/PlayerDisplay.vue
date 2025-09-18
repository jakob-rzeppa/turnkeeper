<script lang="ts" setup>
import { ref, shallowRef } from 'vue'

import { usePlayerStore } from '@/stores/playerStore'
import { socket } from '@/util/connection'
import { useModalStore } from '@/stores/modalStore'
import PlayerEditorModal from './modal/PlayerEditorModal.vue'

const newPlayerName = ref('')

const playerStore = usePlayerStore()
const modalStore = useModalStore()

function createPlayer() {
    if (newPlayerName.value.trim()) {
        socket.emit('players:create', { name: newPlayerName.value.trim() })
        newPlayerName.value = ''
    }
}

function openPlayerEditor(playerId: string) {
    const player = playerStore.players.find((p) => p.id === playerId)
    if (player) {
        const playerEditorModal = shallowRef(PlayerEditorModal)
        modalStore.openModal(playerEditorModal, { player })
    }
}
</script>

<template>
    <hr />
    <div
        v-for="player in playerStore.players"
        :key="player.id"
        @click="openPlayerEditor(player.id)"
    >
        <h3>{{ player.name }}</h3>
        <p>Secret: {{ player.secret }}</p>
        <ul>
            <li v-for="(stat, index) in player.stats" :key="index">
                {{ stat }}
            </li>
        </ul>
        <hr />
    </div>
    <div v-if="playerStore.players.length === 0">
        <p>No players to display.</p>
    </div>
    <div>
        <input type="text" v-model="newPlayerName" placeholder="Enter player name" />
        <button @click="createPlayer">Create Player</button>
    </div>
</template>
