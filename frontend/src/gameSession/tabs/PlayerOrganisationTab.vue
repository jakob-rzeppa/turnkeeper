<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useModalStore } from '../../common/modal/modalStore';
import AttatchUserToPlayerModal from '../modals/AttatchUserToPlayerModal.vue';
import { useUsersStore } from '../../users/usersStore';
import { useCommandEmitter } from '../useCommandEmitter';
import { useSession } from '../useSession';

const commandEmitter = useCommandEmitter();
const session = useSession();
const modalStore = useModalStore();
const usersStore = useUsersStore();
const currentPlayerIndex = computed(() => session.gameState.value?.current_player_index ?? -1);

// Local reorderable copy of players
const localPlayers = ref<{
    name: string;
    user_id: string | null;
}[]>([]);
watch(
    () => session.gameState.value?.players,
    players => {
        localPlayers.value = players ? [...players] : [];
    },
    { immediate: true }
);

// Track whether the order has been changed locally
const orderChanged = computed(() => {
    const original = session.gameState.value?.players ?? [];
    if (original.length !== localPlayers.value.length) return false;
    return original.some((p, i) => p.name !== localPlayers.value[i]?.name);
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
    const names = localPlayers.value.map(p => p.name);
    commandEmitter.changePlayerOrder(names);
};

const addPlayer = () => {
    commandEmitter.addPlayer();
};

const openAttachUserModal = (player: string) => {
    modalStore.openModal(AttatchUserToPlayerModal, { player });
};

const detachUserFromPlayer = (player: string) => {
    commandEmitter.detachUserFromPlayer(player);
};
</script>

<template>
    <div class="w-full h-full p-4 overflow-y-scroll">
        <h2 class="text-lg font-semibold mb-3">Players</h2>
        <ul v-if="localPlayers.length > 0" class="flex flex-col gap-2">
            <li
                v-for="(player, index) in localPlayers"
                :key="player.name"
                class="relative flex items-center gap-2"
            >
                <span class="text-sm font-bold text-base-content/40 w-6 text-center shrink-0">{{
                    index + 1
                }}</span>
                <div
                    draggable="true"
                    class="flex items-center gap-3 flex-1 cursor-grab active:cursor-grabbing select-none transition-opacity relative bg-base-200 rounded-xl p-3"
                    :class="[
                        dragIndex === index ? 'opacity-40' : '',
                        index === currentPlayerIndex ? 'ring-2 ring-accent' : '',
                    ]"
                    @dragstart="onDragStart(index)"
                    @dragover.prevent="onDragOver(index)"
                    @dragend="onDragEnd"
                >
                    <!-- Is active user -->
                    <span
                        v-if="index === currentPlayerIndex"
                        class="absolute -top-3 left-3 text-xs px-2 py-0.5 rounded-full bg-accent text-accent-content font-semibold"
                    >
                        Active
                    </span>

                    <span class="text-base-content/30 text-lg leading-none">⠿</span>

                    <!-- Player Name -->
                    <span class="flex-1 font-medium text-sm">{{player.name}}</span>

                    <!-- Attached User -->
                    <span v-if="player.user_id" class="text-xs text-base-content/60">
                        {{ usersStore.getById(player.user_id)?.value?.name ?? 'Name not found' }}
                    </span>

                    <!-- Attach/Detach User Button -->
                    <button
                        v-if="!player.user_id"
                        class="btn btn-xs btn-outline"
                        @click="openAttachUserModal(player.name)"
                    >
                        Attach User
                    </button>
                    <button
                        v-else
                        class="btn btn-xs btn-outline text-error"
                        @click="detachUserFromPlayer(player.name)"
                    >
                        Detach User
                    </button>
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
