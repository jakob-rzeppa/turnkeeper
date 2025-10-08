<script lang="ts" setup>
import { shallowRef } from 'vue'

import { usePlayerStore } from '@/stores/playerStore'
import { useModalStore } from '@/stores/modalStore'
import PlayerEditorModal from '../player/PlayerEditorModal.vue'
import DisplayContainer from '../container/DisplayContainer.vue'

const playerStore = usePlayerStore()
const modalStore = useModalStore()

function openPlayerEditor(playerId: string) {
    const playerEditorModal = shallowRef(PlayerEditorModal)
    modalStore.openModal(playerEditorModal, { playerId })
}
</script>

<template>
    <DisplayContainer label="Players" class="col-span-2">
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
    </DisplayContainer>
</template>
