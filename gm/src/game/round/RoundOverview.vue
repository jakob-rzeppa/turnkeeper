<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../gameStore';
import { useUsersStore } from '../../users/usersStore';
import { useEventEmitter } from '../../events/useEventEmitter';
import InlineStatEditor from '../stats/InlineStatEditor.vue';
import AddStatModal from '../stats/AddStatModal.vue';
import { useModalStore } from '../../common/modal/modalStore';
import InlineTradableEditor from '../tradables/InlineTradableEditor.vue';

const gameStore = useGameStore();
const usersStore = useUsersStore();
const eventEmitter = useEventEmitter();
const modalStore = useModalStore();

const currentPlayer = computed(() => {
    const index = gameStore.game?.currentPlayerIndex;
    if (index === undefined || index < 0) return null;
    return gameStore.game?.players[index] ?? null;
});
</script>

<template>
    <h2 class="text-2xl font-bold">Round Overview</h2>
    <div class="flex flex-col gap-4">
        <div>
            <p>Round: {{ gameStore.game?.roundNumber }}</p>
        </div>
        <div class="flex flex-row gap-2 items-center">
            <template v-for="(player, index) in gameStore.game?.players" :key="player.id">
                <button
                    :class="[
                        'btn btn-xs',
                        currentPlayer?.id !== player.id ? 'btn-ghost' : 'btn-secondary',
                    ]"
                    @click="eventEmitter.skipTurnToPlayer(player.id)"
                >
                    {{ usersStore.getPlayerName(player.id) }}
                </button>
                <span v-if="index < (gameStore.game?.players?.length ?? 0) - 1" class="text-lg">
                    →
                </span>
            </template>
        </div>
        <div class="flex flex-row gap-2">
            <button class="btn btn-sm btn-ghost btn-warning" @click="eventEmitter.previousTurn">
                Previous Turn
            </button>
            <button class="btn btn-sm btn-primary" @click="eventEmitter.nextTurn">Next Turn</button>
        </div>
    </div>

    <div v-if="!currentPlayer">
        <p class="text-base-content/40 italic">No active player.</p>
    </div>
    <div v-else>
        <div class="divider">Tradables</div>
        <InlineTradableEditor
            v-for="tradable in currentPlayer.tradables"
            :key="tradable.id"
            :playerId="currentPlayer.id"
            :tradable="tradable"
            size="lg"
        />
        <div class="divider">Stats</div>
        <div class="flex flex-col gap-2 items-center">
            <InlineStatEditor
                v-for="stat in currentPlayer.stats"
                :key="stat.id"
                :playerId="currentPlayer.id"
                :stat="stat"
                size="lg"
            />

            <button
                class="btn btn-lg btn-circle btn-primary"
                title="Add Stat"
                @click="modalStore.openModal(AddStatModal, { playerId: currentPlayer.id })"
            >
                +
            </button>
        </div>
    </div>
</template>
