<script setup lang="ts">
import { onUnmounted, ref, watch } from 'vue'
import { useAutosaveObjectEditor } from '@/composables/useAutosaveObjectEditor'
import { usePlayerStore } from '@/stores/playerStore'
import type { Player } from 'shared-types'
import { usePlayerEmitter } from '@/emitters/playerEmitter'
import { useModalStore } from '@/stores/modalStore'
import NewStatModal from './NewStatModal.vue'

const props = defineProps<{
    playerId: number
}>()

const playerStore = usePlayerStore()
const modalStore = useModalStore()
const playerEmitter = usePlayerEmitter()

const player = ref<Player | undefined>(undefined)

watch(
    () => playerStore.getPlayerById(props.playerId),
    (newPlayer) => {
        if (newPlayer) {
            player.value = newPlayer
        }
    },
    { immediate: true, deep: true },
)

const { editableObject, areEditableObjectFieldsChanged, handleFieldInput, saveChanges } =
    useAutosaveObjectEditor<{ [keyof: string]: string }>(
        () => {
            const statsRecord: { [keyof: string]: string } = {}
            player.value?.stats.forEach((stat) => {
                statsRecord[stat.id.toString()] = stat.value
            })
            return statsRecord
        },
        (newStats) => {
            Object.keys(newStats).forEach((statId: string) => {
                playerEmitter.updateStatValueForPlayer(
                    props.playerId,
                    parseInt(statId),
                    newStats[statId],
                )
            })
        },
    )

onUnmounted(() => {
    saveChanges()
})
</script>

<template>
    <div v-if="!player">Player with Id {{ props.playerId }} not found</div>
    <div v-else class="card bg-base-100 border border-secondary/20">
        <div class="card-body">
            <div class="card-title text-secondary mb-4 flex items-center justify-between">
                <span>Player Stats</span>
                <div class="badge badge-secondary badge-outline">
                    {{ player.stats.length }}
                </div>
            </div>

            <div v-if="player.stats.length > 0" class="space-y-3">
                <div
                    v-for="stat in player.stats"
                    :key="stat.id"
                    class="flex gap-3 items-center p-3 bg-base-200 rounded-lg"
                >
                    <label
                        @focusout="saveChanges"
                        @keypress="(e) => (e.key === 'Enter' ? saveChanges() : null)"
                        :class="`input input-bordered input-sm w-full ${areEditableObjectFieldsChanged[stat.id] ? 'input-primary' : ''}`"
                    >
                        <span class="label">{{
                            stat.name + (areEditableObjectFieldsChanged[stat.id] ? '*' : '')
                        }}</span>
                        <input
                            type="text"
                            :value="editableObject[stat.id]"
                            @input="(e: Event) => handleFieldInput(stat.id, e)"
                            :placeholder="`Enter ${stat.name}...`"
                        />
                    </label>
                    <button
                        class="btn btn-error btn-sm btn-circle"
                        @click="playerEmitter.removeStatFromPlayer(props.playerId, stat.id)"
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

            <div class="card-actions">
                <button
                    class="btn btn-secondary btn-outline w-full"
                    @click="
                        () =>
                            modalStore.openModal(NewStatModal, {
                                playerId: props.playerId,
                                playerName: player?.name,
                            })
                    "
                >
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
