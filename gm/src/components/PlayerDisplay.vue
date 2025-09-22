<script lang="ts" setup>
import { shallowRef } from 'vue'

import { usePlayerStore } from '@/stores/playerStore'
import { useModalStore } from '@/stores/modalStore'
import PlayerEditorModal from './modal/PlayerEditorModal.vue'
import NewPlayerModal from './modal/NewPlayerModal.vue'

const playerStore = usePlayerStore()
const modalStore = useModalStore()

function openPlayerEditor(playerId: string) {
    const player = playerStore.players.find((p) => p.id === playerId)

    const playerEditorModal = shallowRef(PlayerEditorModal)
    modalStore.openModal(playerEditorModal, { playerId })
}

function openNewPlayerModal() {
    const newPlayerModal = shallowRef(NewPlayerModal)
    modalStore.openModal(newPlayerModal)
}
</script>

<template>
    <div class="p-4 border rounded-sm bg-base-200">
        <h2 class="text-3xl font-bold">Players</h2>
        <div class="flex flex-row gap-4 p-4">
            <div
                v-for="player in playerStore.players"
                :key="player.id"
                @click="openPlayerEditor(player.id)"
                class="card cursor-pointer hover:shadow-lg transition-shadow bg-base-100 card-lg border-primary border rounded-sm"
            >
                <div class="card-body">
                    <h3 class="text-3xl font-bold card-title text-primary">{{ player.name }}</h3>
                    <p class="badge badge-sm">Secret: {{ player.secret }}</p>
                    <div class="flex flex-col gap-2">
                        <h4 class="text-lg text-primary">Stats</h4>
                        <div class="input w-full" v-for="stat in player.stats" :key="stat.name">
                            <span class="label">{{ stat.name }}</span>
                            <span>{{ stat.value }}</span>
                        </div>
                    </div>
                </div>
            </div>
            <div v-if="playerStore.players.length === 0">
                <p>No players to display.</p>
            </div>
        </div>
        <button class="btn btn-secondary" @click="openNewPlayerModal">Create Player</button>
    </div>
</template>
