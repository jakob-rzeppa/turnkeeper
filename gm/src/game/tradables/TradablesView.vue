<script setup lang="ts">
import { useCommandEmitter } from '../../commands/useCommandEmitter';
import { useGameStore } from '../gameStore';
import { computed, ref } from 'vue';
import CreateTradableModal from './CreateTradableModal.vue';
import { useModalStore } from '../../common/modal/modalStore';
import { useUsersStore } from '../../users/usersStore';
import DeleteTradableModal from './DeleteTradableModal.vue';

const commandEmitter = useCommandEmitter();
const gameStore = useGameStore();
const modalStore = useModalStore();

// Helper to get player names
const usersStore = useUsersStore();

// State for current tradable selection
const currentTradableIndex = ref(0);

// Get all unique tradables from all players
const allTradables = computed(() => {
    const game = gameStore.game;
    if (!game || !game.players) {
        return [];
    }
    const tradableMap = new Map<string, { id: string; name: string }>();
    game.players.forEach(player => {
        player.tradables?.forEach(tradable => {
            if (!tradableMap.has(tradable.id)) {
                tradableMap.set(tradable.id, { id: tradable.id, name: tradable.name });
            }
        });
    });
    return Array.from(tradableMap.values());
});

const currentTradable = computed(() => {
    if (allTradables.value.length === 0) {
        return null;
    }
    return allTradables.value[currentTradableIndex.value];
});

const updatePlayerTradableValue = (playerId: string, newValue: number) => {
    if (!currentTradable.value) {
        return;
    }
    commandEmitter.changePlayerTradableValue(playerId, currentTradable.value.id, newValue);
};

// Sending tradables between players
const fromPlayerId = ref<string | null>(null);
const toPlayerId = ref<string | null>(null);
const tradableSendAmount = ref(0);

const sendTradableFromPlayerToPlayer = () => {
    if (!currentTradable.value) {
        alert('Please select a tradable to send.');
        return;
    }
    if (!fromPlayerId.value || !toPlayerId.value) {
        alert('Please select both a sender and a receiver.');
        return;
    }
    commandEmitter.sendTradable(
        fromPlayerId.value,
        toPlayerId.value,
        currentTradable.value?.id,
        tradableSendAmount.value
    );
};
</script>

<template>
    <div class="p-4 md:p-6">
        <div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
            <div>
                <h2 class="text-2xl font-bold">Tradables</h2>
                <p class="text-sm text-base-content/60">Manage values and send between players.</p>
            </div>
            <button
                class="btn btn-primary"
                @click="modalStore.openModal(CreateTradableModal, { title: 'Create Tradable' })"
            >
                Create New Tradable
            </button>
        </div>

        <div class="mt-6">
            <div v-if="allTradables.length === 0" class="alert">
                <span>No tradables created yet.</span>
            </div>

            <div v-else class="space-y-6">
                <div class="tabs tabs-boxed bg-base-200/70 inline-flex">
                    <button
                        v-for="tradable in allTradables"
                        :key="tradable.id"
                        @click="currentTradableIndex = allTradables.indexOf(tradable)"
                        :class="['tab', currentTradable?.id === tradable.id ? 'tab-active' : '']"
                    >
                        {{ tradable.name }}
                    </button>
                </div>

                <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
                    <div class="card bg-base-100 shadow-sm border border-base-200">
                        <div class="card-body">
                            <div class="flex items-center justify-between">
                                <h3 class="card-title">Player Values</h3>
                                <div class="badge badge-outline badge-secondary">
                                    {{ currentTradable?.name || '—' }}
                                </div>
                            </div>
                            <div class="overflow-x-auto">
                                <table class="table table-sm">
                                    <thead>
                                        <tr>
                                            <th>Player</th>
                                            <th class="text-right">Value</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr
                                            v-for="player in gameStore.game?.players || []"
                                            :key="player.id"
                                        >
                                            <td>{{ usersStore.getPlayerName(player.userId) }}</td>
                                            <td class="text-right">
                                                <input
                                                    type="number"
                                                    class="input input-bordered w-32 text-right"
                                                    :value="
                                                        player.tradables?.find(
                                                            t => t.id === currentTradable?.id
                                                        )?.value || 0
                                                    "
                                                    @change="
                                                        e =>
                                                            updatePlayerTradableValue(
                                                                player.id,
                                                                Number(
                                                                    (e.target as HTMLInputElement)
                                                                        .value
                                                                )
                                                            )
                                                    "
                                                />
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>

                    <div class="card bg-base-100 shadow-sm border border-base-200">
                        <div class="card-body">
                            <h3 class="card-title">Send Tradable</h3>
                            <form
                                class="space-y-3"
                                @submit.prevent="sendTradableFromPlayerToPlayer"
                            >
                                <label class="select select-bordered w-full">
                                    <span class="label">From</span>
                                    <select v-model="fromPlayerId">
                                        <option disabled value="">Select player</option>
                                        <option
                                            v-for="player in gameStore.game?.players || []"
                                            :key="player.id"
                                            :value="player.id"
                                        >
                                            {{ usersStore.getPlayerName(player.userId) }}
                                        </option>
                                    </select>
                                </label>
                                <label class="select select-bordered w-full">
                                    <span class="label">To</span>
                                    <select v-model="toPlayerId">
                                        <option disabled value="">Select player</option>
                                        <option
                                            v-for="player in gameStore.game?.players || []"
                                            :key="player.id"
                                            :value="player.id"
                                        >
                                            {{ usersStore.getPlayerName(player.userId) }}
                                        </option>
                                    </select>
                                </label>
                                <label class="input input-bordered w-full">
                                    <span class="label">Amount</span>
                                    <input
                                        v-model.number="tradableSendAmount"
                                        type="number"
                                        placeholder="Enter value"
                                    />
                                </label>
                                <div class="card-actions justify-end pt-2">
                                    <button class="btn btn-primary" type="submit">Send</button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>

                <button
                    v-if="currentTradable"
                    class="btn btn-error btn-ghost mt-4"
                    @click="
                        modalStore.openModal(DeleteTradableModal, {
                            tradableId: currentTradable.id,
                            tradableName: currentTradable.name,
                        })
                    "
                >
                    Delete {{ currentTradable.name }}
                </button>
            </div>
        </div>
    </div>
</template>
