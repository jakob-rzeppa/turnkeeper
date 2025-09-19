<script lang="ts" setup>
import { ref, shallowRef } from 'vue'

import { usePlayerStore } from '@/stores/playerStore'
import { useModalStore } from '@/stores/modalStore'
import PlayerEditorModal from './modal/PlayerEditorModal.vue'
import NewPlayerModal from './modal/NewPlayerModal.vue'

const playerStore = usePlayerStore()
const modalStore = useModalStore()

function openPlayerEditor(playerId: string) {
    const player = playerStore.players.find((p) => p.id === playerId)
    if (player) {
        const playerEditorModal = shallowRef(PlayerEditorModal)
        modalStore.openModal(playerEditorModal, { player })
    }
}

function openNewPlayerModal() {
    const newPlayerModal = shallowRef(NewPlayerModal)
    modalStore.openModal(newPlayerModal)
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
    <button class="btn" @click="openNewPlayerModal">Create Player</button>
</template>
