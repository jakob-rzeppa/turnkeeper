<script setup lang="ts">
import type { Player } from '@/types/player'
import { socket } from '@/util/connection'
import { ref, shallowRef, watch } from 'vue'
import NewStatModal from './NewStatModal.vue'
import { useModalStore } from '@/stores/modalStore'
import { usePlayerStore } from '@/stores/playerStore'

// The Player prop needs to be a deep clone
const props = defineProps<{
    player: Player
}>()

const emit = defineEmits(['close'])

const modalStore = useModalStore()
const playerStore = usePlayerStore()

/*
 * Even though we update the refs if the playerStore changes we get the initial values from the props.
 * That way we don't have to deal with a possibly undefined player while the modal is open.
 * If the player is deleted while the modal is open, we just close the modal (see next watch).
 */
const playerNameRef = ref(props.player.name)
const playerSecretRef = ref(props.player.secret)
const playerStatsRef = ref(props.player.stats)

// Update Player info, when the player in the backend changes
watch(
    () => playerStore.players,
    () => {
        const player = playerStore.players.find((p) => p.id === props.player.id)

        // When the player is not found (deleted), close the modal
        if (!player) {
            emit('close')
            return
        }

        // Only change the values that are different to avoid overwriting user input. All values need to be copied if they aren't primitive.
        if (player.name !== playerNameRef.value) playerNameRef.value = player.name
        if (player.secret !== playerSecretRef.value) playerSecretRef.value = player.secret
        for (const stat of player.stats) {
            const existingStat = playerStatsRef.value.find((s) => s.name === stat.name)
            if (existingStat) {
                if (existingStat.value !== stat.value) existingStat.value = stat.value
            } else {
                playerStatsRef.value.push({ ...stat })
            }
        }
    },
    { deep: true },
)

function updatePlayer(): void {
    socket.emit('players:update', {
        playerId: props.player.id,
        playerData: {
            name: playerNameRef.value,
            secret: playerSecretRef.value,
            stats: playerStatsRef.value,
        },
    })
    emit('close')
}

function deletePlayer(): void {
    if (
        confirm(
            `Are you sure you want to delete player ${props.player.name}? This action cannot be undone.`,
        )
    ) {
        socket.emit('players:delete', { playerId: props.player.id })
        emit('close')
    }
}

function openNewStatModal(): void {
    const newStatModal = shallowRef(NewStatModal)
    modalStore.openModal(newStatModal, { playerId: props.player.id, playerName: props.player.name })
    // Changes via the NewStatModal will be reflected in the playerStatsRef via the watch above
}

function removeStatFromPlayer(statName: string): void {
    socket.emit('players:stats:remove', { playerId: props.player.id, statName })
    // The playerStore will be updated via the socket event, which will also update the playerStatsRef via the watch above
}
</script>

<template>
    <h1 class="text-4xl text-center text-primary">Edit player</h1>
    <label class="input input-primary w-full">
        <span class="label">Name</span>
        <input type="text" v-model="playerNameRef" />
    </label>
    <label class="input input-primary w-full">
        <span class="label">Secret</span>
        <input type="text" v-model="playerSecretRef" />
    </label>
    <div class="flex flex-col gap-2 p-4 border border-secondary rounded-lg">
        <h2 class="text-2xl text-center text-secondary">Stats</h2>
        <div class="w-full flex flex-row gap-2" v-for="stat in playerStatsRef" :key="stat.name">
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
    <button class="btn btn-primary btn-lg" @click="updatePlayer">Update Player</button>
    <button class="btn btn-error btn-sm btn-outline" @click="deletePlayer">Delete Player</button>
</template>
