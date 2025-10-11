<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import type { PlayerStat } from '@/types/player'
import { ref } from 'vue'

const props = defineProps<{
    playerId?: string
    playerName?: string
}>()

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()

const scopeRef = ref<'global' | 'player'>(props.playerId ? 'player' : 'global')
const statNameRef = ref('')
const statInitialValueRef = ref('')

const createStat = () => {
    const statData: PlayerStat = {
        name: statNameRef.value,
        value: statInitialValueRef.value,
    }

    playerStore.createStatForPlayer(statData, scopeRef.value, props.playerId)

    emit('close')
}
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Add New Stat</h1>
            <p class="text-base-content/70">Create a new statistic for tracking</p>
        </div>

        <div class="form-control">
            <label class="label">
                <span class="label-text font-medium">Scope</span>
                <span class="label-text-alt">Where should this stat be applied?</span>
            </label>
            <select v-model="scopeRef" class="select select-bordered select-primary w-full">
                <option value="global">Global (All Players)</option>
                <option value="player" :disabled="!props.playerId">
                    Player: {{ props.playerName ?? '(unknown)' }}
                </option>
            </select>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Stat Name</span>
                </label>
                <input
                    type="text"
                    v-model="statNameRef"
                    class="input input-bordered input-primary w-full"
                    placeholder="e.g., Health, Mana, Level..."
                    @keyup.enter="createStat"
                />
            </div>

            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">Initial Value</span>
                </label>
                <input
                    type="text"
                    v-model="statInitialValueRef"
                    class="input input-bordered input-secondary w-full"
                    placeholder="e.g., 100, 0, Beginner..."
                    @keyup.enter="createStat"
                />
            </div>
        </div>

        <div class="modal-action">
            <button
                class="btn btn-primary btn-lg w-full"
                @click="createStat"
                :disabled="!statNameRef.trim() || !statInitialValueRef.trim()"
            >
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                    ></path>
                </svg>
                Add Stat
            </button>
        </div>
    </div>
</template>
