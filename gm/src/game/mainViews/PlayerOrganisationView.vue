<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useGameStore, type Player } from '../../game/gameStore';
import { useEventEmitter } from '../../events/useEventEmitter';

const { emit } = useEventEmitter();
const gameStore = useGameStore();
const currentPlayerIndex = computed(() => gameStore.game?.currentPlayerIndex ?? -1);

// Local reorderable copy of players
const localPlayers = ref<Player[]>([]);
watch(
    () => gameStore.game?.players,
    players => {
        localPlayers.value = players ? [...players] : [];
    },
    { immediate: true }
);

// Track whether the order has been changed locally
const orderChanged = computed(() => {
    const original = gameStore.game?.players ?? [];
    if (original.length !== localPlayers.value.length) return false;
    return original.some((p, i) => p.id !== localPlayers.value[i]?.id);
});

// Drag state
const dragIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

const onDragStart = (index: number) => {
    dragIndex.value = index;
};

const onDragOver = (index: number) => {
    if (dragIndex.value === null || dragIndex.value === index) return;
    dragOverIndex.value = index;

    const items = [...localPlayers.value];
    const [moved] = items.splice(dragIndex.value, 1);
    if (!moved) return;
    items.splice(index, 0, moved);
    localPlayers.value = items;
    dragIndex.value = index;
};

const onDragEnd = () => {
    dragIndex.value = null;
    dragOverIndex.value = null;
};

const updateOrder = () => {
    const ids = localPlayers.value.map(p => p.id);
    emit({ ChangePlayerOrder: ids });
};

const addPlayer = () => {
    emit('AddPlayer');
};
</script>

<template>
    <div class="w-full h-full p-4 overflow-y-scroll">
        <h2 class="text-lg font-semibold mb-3">Players</h2>
        <ul v-if="localPlayers.length > 0" class="flex flex-col gap-2">
            <li
                v-for="(player, index) in localPlayers"
                :key="player.id"
                class="flex items-center gap-3"
            >
                <span class="text-sm font-medium text-gray-500 w-5 text-right">{{
                    index + 1
                }}</span>
                <div
                    draggable="true"
                    class="flex items-center gap-3 p-3 rounded-lg border bg-base-200 cursor-grab active:cursor-grabbing select-none transition-opacity flex-1"
                    :class="[
                        index === currentPlayerIndex ? 'border-accent' : 'border-base-300',
                        dragIndex === index ? 'opacity-50' : '',
                    ]"
                    @dragstart="onDragStart(index)"
                    @dragover.prevent="onDragOver(index)"
                    @dragend="onDragEnd"
                >
                    <span class="mr-1 text-primary">⠿</span>
                    <span class="flex-1">{{ player.user?.name ?? 'Unassigned' }}</span>
                    <span>{{ player.id }}</span>
                    <span
                        v-if="index === currentPlayerIndex"
                        class="text-xs px-2 py-0.5 rounded-full bg-accent text-accent-content"
                    >
                        Active
                    </span>
                </div>
            </li>
        </ul>
        <p v-else class="text-sm text-center">No players yet.</p>
        <div class="flex items-center justify-center gap-2 mt-2">
            <button v-if="orderChanged" class="btn btn-secondary" @click="updateOrder">
                Update Order
            </button>
            <button v-else class="btn btn-primary btn-circle" @click="addPlayer">+</button>
        </div>
    </div>
</template>
