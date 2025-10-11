<script setup lang="ts">
import PlayerStatsEditor from './PlayerStatsEditor.vue'
import { usePlayerEditor } from '@/composables/usePlayerEditor'

const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['done'])

const { localPlayer, updatePlayer } = usePlayerEditor(props.playerId, () => emit('done'))
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Edit Player</h1>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Player Name</span>
                </label>
                <input
                    type="text"
                    v-model="localPlayer.name"
                    class="input input-bordered input-primary w-full"
                    placeholder="Enter player name..."
                />
            </div>

            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Secret Code</span>
                </label>
                <input
                    type="text"
                    v-model="localPlayer.secret"
                    class="input input-bordered input-secondary w-full"
                    placeholder="Enter secret code..."
                />
            </div>
        </div>

        <PlayerStatsEditor
            :player-id="props.playerId"
            :player-name="localPlayer.name"
            :player-stats="localPlayer.stats"
        />

        <div class="modal-action">
            <button class="btn btn-primary btn-lg w-full" @click="updatePlayer">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M5 13l4 4L19 7"
                    ></path>
                </svg>
                Update Player
            </button>
        </div>
    </div>
</template>
