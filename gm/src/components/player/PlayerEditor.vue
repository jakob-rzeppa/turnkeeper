<script setup lang="ts">
import { useModalStore } from '@/stores/modalStore'
import { usePlayerStore } from '@/stores/playerStore'
import { ref, shallowRef, watch } from 'vue'
import NewStatModal from '../modal/NewStatModal.vue'
import useConnection from '@/composables/connection'

const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['done'])

const modalStore = useModalStore()
const playerStore = usePlayerStore()
const { socket } = useConnection()

/*
 * Even though we update the refs if the playerStore changes we get the initial values from the props.
 * That way we don't have to deal with a possibly undefined player while the modal is open.
 */
const playerNameRef = ref('')
const playerSecretRef = ref('')
const playerStatsRef = ref<{ name: string; value: boolean | number | string | string[] }[]>([])

watch(
    () => props.playerId,
    (playerId) => {
        const newPlayer = playerStore.getPlayerById(playerId)
        console.log('Loaded player:', newPlayer)
        if (!newPlayer) return

        playerNameRef.value = newPlayer.name
        playerSecretRef.value = newPlayer.secret
        playerStatsRef.value = newPlayer.stats.map((s) => ({ ...s }))
    },
    { immediate: true },
)

// Update Player info, when the player in the backend changes
watch(
    () => playerStore.players,
    () => {
        const updatedPlayer = playerStore.getPlayerById(props.playerId)

        // When the player is not found (deleted), close the modal
        if (!updatedPlayer) {
            emit('done')
            return
        }

        playerNameRef.value = updatedPlayer.name
        playerSecretRef.value = updatedPlayer.secret
        playerStatsRef.value = updatedPlayer.stats.map((s) => ({ ...s }))
    },
    { deep: true },
)

function updatePlayer(): void {
    playerStore.updatePlayer(props.playerId, {
        name: playerNameRef.value,
        secret: playerSecretRef.value,
        stats: playerStatsRef.value,
    })
    emit('done')
}

function openNewStatModal(): void {
    const newStatModal = shallowRef(NewStatModal)
    modalStore.openModal(newStatModal, {
        playerId: props.playerId,
        playerName: playerNameRef.value,
    })
    // Changes via the NewStatModal will be reflected in the playerStatsRef via the watch above
}

function removeStatFromPlayer(statName: string): void {
    playerStore.removeStatFromPlayer(props.playerId, statName)
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
</template>
