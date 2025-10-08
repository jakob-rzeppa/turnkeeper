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
    <h1 class="text-4xl text-center text-primary">Add new stat</h1>
    <label class="select select-primary">
        <span class="label">Scope</span>
        <select v-model="scopeRef">
            <option value="global">Global</option>
            <option value="player" :disabled="!props.playerId">
                Player {{ props.playerName ?? '(unknown)' }}
            </option>
        </select>
    </label>
    <label class="input input-primary">
        <span class="label">Name</span>
        <input type="text" v-model="statNameRef" />
    </label>
    <label class="input input-primary">
        <span class="label">Initial Value</span>
        <input type="text" v-model="statInitialValueRef" />
    </label>
    <button class="btn btn-primary" @click="createStat">Add Stat</button>
</template>
