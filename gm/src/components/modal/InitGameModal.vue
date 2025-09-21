<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import { socket } from '@/util/connection'
import { ref } from 'vue'
import draggable from 'vuedraggable'

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()

const playerOrderRef = ref<{ id: string; name: string }[]>([])

playerOrderRef.value = playerStore.players.map((player) => ({
    id: player.id,
    name: player.name,
}))

function startGame() {
    socket.emit('game:init', { playerIdsInOrder: playerOrderRef.value.map((p) => p.id) })
    emit('close')
}
</script>

<template>
    <h2 class="text-2xl font-bold text-center">Init Game</h2>
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
    <button class="btn btn-primary mt-4 w-full" @click="startGame">Start Game</button>
</template>
