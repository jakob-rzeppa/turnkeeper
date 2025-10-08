<script setup lang="ts">
import useConnection from '@/composables/connection'
import { useGameStore } from '@/stores/gameStore'
import { ref } from 'vue'
import draggable from 'vuedraggable'

const emit = defineEmits(['close'])

const gameStore = useGameStore()
const { socket } = useConnection()

const playerOrderRef = ref<{ id: string; name: string }[]>([])

playerOrderRef.value = [...gameStore.playerOrder]

function startGame() {
    socket.emit('game:playerOrder:update', {
        playerIdsInOrder: playerOrderRef.value.map((p) => p.id),
    })
    emit('close')
}
</script>

<template>
    <h2 class="text-2xl font-bold text-center">Update Player Order</h2>
    <p class="text-center mb-4">Drag and drop to set the player order</p>
    <draggable v-model="playerOrderRef" item-key="id" class="list bg-base-100 shadow-md rounded-sm">
        <template #item="{ element: player, index }">
            <li class="list-row cursor-pointer" :key="index">
                <div class="list-col-grow">
                    <div>{{ player.name }}</div>
                    <div class="text-xs uppercase font-semibold opacity-60">
                        {{ player.id }}
                    </div>
                </div>
            </li>
        </template>
    </draggable>
    <button class="btn btn-primary mt-4 w-full" @click="startGame">Update Player Order</button>
</template>
