<script lang="ts" setup>
import { usePlayerStore } from '@/stores/playerStore'
import { socket } from '@/util/connection'
import { ref } from 'vue'

const newPlayerName = ref('')

const playerStore = usePlayerStore()

function createPlayer() {
    if (newPlayerName.value.trim()) {
        socket.emit('players:create', { name: newPlayerName.value.trim() })
        newPlayerName.value = ''
    }
}
</script>

<template>
    <hr />
    <div v-for="player in playerStore.players" :key="player.name">
        <h3>{{ player.name }}</h3>
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
