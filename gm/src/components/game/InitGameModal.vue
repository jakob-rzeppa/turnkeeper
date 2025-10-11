<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore'
import { usePlayerStore } from '@/stores/playerStore'
import { ref } from 'vue'
import draggable from 'vuedraggable'

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()
const gameStore = useGameStore()

const playerOrderRef = ref<{ id: string; name: string }[]>([])

playerOrderRef.value = playerStore.players.map((player) => ({
    id: player.id,
    name: player.name,
}))

function startGame() {
    gameStore.initGame(playerOrderRef.value.map((p) => p.id))
    emit('close')
}
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h2 class="text-3xl font-bold text-primary mb-2">Initialize Game</h2>
            <p class="text-base-content/70">Drag and drop to set the player turn order</p>
        </div>

        <div class="bg-base-200 p-4 rounded-lg">
            <h3 class="text-lg font-semibold mb-3 text-secondary">Player Turn Order</h3>
            <draggable
                v-model="playerOrderRef"
                item-key="id"
                class="space-y-2"
                ghost-class="opacity-50"
                chosen-class="scale-105"
            >
                <template #item="{ element: player, index }">
                    <div
                        class="card bg-base-100 shadow-sm border border-base-300 cursor-move hover:shadow-md transition-all"
                    >
                        <div class="card-body p-4 flex-row items-center">
                            <div class="badge badge-primary mr-3">{{ index + 1 }}</div>
                            <div class="flex-1">
                                <h4 class="font-medium">{{ player.name }}</h4>
                                <p class="text-xs text-base-content/60">ID: {{ player.id }}</p>
                            </div>
                            <svg
                                class="w-5 h-5 text-base-content/50"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"
                                ></path>
                            </svg>
                        </div>
                    </div>
                </template>
            </draggable>
        </div>

        <div>
            <button
                class="btn btn-primary btn-lg w-full"
                @click="startGame"
                :disabled="playerOrderRef.length === 0"
            >
                Start Game
            </button>
        </div>
    </div>
</template>
