<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import { shallowRef } from 'vue'
import NewStatModal from '../modal/NewStatModal.vue'
import { usePlayerStore } from '@/stores/playerStore'

const props = defineProps<{
    playerId: string
    playerName: string
    playerStats: { name: string; value: boolean | number | string | string[] }[]
}>()

const modalStore = useModalStore()
const playerStore = usePlayerStore()

function openNewStatModal(): void {
    const newStatModal = shallowRef(NewStatModal)
    modalStore.openModal(newStatModal, {
        playerId: props.playerId,
        playerName: props.playerName,
    })
    // Changes via the NewStatModal will be reflected in the playerStatsRef via the watch above
}

function removeStatFromPlayer(statName: string): void {
    playerStore.removeStatFromPlayer(props.playerId, statName)
}
</script>

<template>
    <div class="flex flex-col gap-2 p-4 border border-secondary rounded-lg">
        <h2 class="text-2xl text-center text-secondary">Stats</h2>
        <div class="w-full flex flex-row gap-2" v-for="stat in props.playerStats" :key="stat.name">
            <label class="input w-full">
                <span class="label">{{ stat.name }}</span>
                <input type="text" v-model="stat.value" />
            </label>
            <button
                class="btn btn-secondary bg-base-100 btn-outline"
                @click="removeStatFromPlayer(stat.name)"
            >
                x
            </button>
        </div>
        <button class="btn btn-secondary" @click="openNewStatModal">Add Stat</button>
    </div>
</template>
