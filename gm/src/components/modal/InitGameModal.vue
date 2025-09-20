<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'
import { ref } from 'vue'
import draggable from 'vuedraggable'

const emit = defineEmits(['close'])

const playerStore = usePlayerStore()

const playerOrderRef = ref<{ id: string; name: string }[]>([])

playerOrderRef.value = playerStore.players.map((player) => ({
    id: player.id,
    name: player.name,
}))
</script>

<template>
    <h1 class="text-2xl font-bold text-center">Init Game</h1>
    <draggable
        v-model="playerOrderRef"
        item-key="id"
        class="list bg-base-100 rounded-box shadow-md"
    >
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
</template>
