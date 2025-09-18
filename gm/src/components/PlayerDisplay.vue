<script lang="ts" setup>
import { ref } from 'vue'

import { usePlayerStore } from '@/stores/playerStore'
import { socket } from '@/util/connection'
import type { Player } from '@/types/player'

const newPlayerName = ref('')

const playerStore = usePlayerStore()

function createPlayer() {
    if (newPlayerName.value.trim()) {
        socket.emit('players:create', { name: newPlayerName.value.trim() })
        newPlayerName.value = ''
    }
}

const playerToEdit = ref(null as null | Player)

function openPlayerEditor(playerName: string) {
    const player = playerStore.players.find((p) => p.name === playerName)
    if (player) {
        playerToEdit.value = player
    }
}
</script>

<template>
    <hr />
    <div
        v-for="player in playerStore.players"
        :key="player.name"
        @click="openPlayerEditor(player.name)"
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
