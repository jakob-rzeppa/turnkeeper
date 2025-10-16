<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import NewStatModal from './NewStatModal.vue'
import { usePlayerEmitter } from '@/emitters/playerEmitter'
import type { PlayerStat } from 'shared-types'

const props = defineProps<{
    playerId: number
    playerName: string
    playerStats: PlayerStat[]
}>()

const modalStore = useModalStore()
const playerEmitter = usePlayerEmitter()

function openNewStatModal(): void {
    modalStore.openModal(NewStatModal, {
        playerId: props.playerId,
        playerName: props.playerName,
    })
}

function removeStatFromPlayer(statId: number): void {
    playerEmitter.removeStatFromPlayer(props.playerId, statId)
}
</script>

<template>
    <div class="card bg-base-100 border border-secondary/20">
        <div class="card-body">
            <div class="card-title text-secondary mb-4 flex items-center justify-between">
                <span>Player Stats</span>
                <div class="badge badge-secondary badge-outline">
                    {{ props.playerStats.length }}
                </div>
            </div>

            <div v-if="props.playerStats.length > 0" class="space-y-3">
                <div
                    v-for="stat in props.playerStats"
                    :key="stat.name"
                    class="flex gap-3 items-center p-3 bg-base-200 rounded-lg"
                >
                    <div class="flex-1">
                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text font-medium">{{ stat.name }}</span>
                            </div>
                            <input
                                type="text"
                                v-model="stat.value"
                                class="input input-bordered input-sm w-full"
                                :placeholder="`Enter ${stat.name}...`"
                            />
                        </label>
                    </div>
                    <button
                        class="btn btn-error btn-sm btn-circle"
                        @click="removeStatFromPlayer(stat.id)"
                        :title="`Remove ${stat.name}`"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>

            <div v-else class="text-center py-6">
                <p class="text-sm text-base-content/60">No stats added yet</p>
            </div>

            <div class="card-actions mt-4">
                <button class="btn btn-secondary btn-outline w-full" @click="openNewStatModal">
                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                        ></path>
                    </svg>
                    Add New Stat
                </button>
            </div>
        </div>
    </div>
</template>
