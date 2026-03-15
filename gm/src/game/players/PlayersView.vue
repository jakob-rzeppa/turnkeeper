<script setup lang="ts">
import { useModalStore } from '../../common/modal/modalStore';
import { useUsersStore } from '../../users/usersStore';
import { useGameStore } from '../gameStore';
import AddStatModal from '../stats/AddStatModal.vue';
import InlineStatsEditor from '../stats/InlineStatsEditor.vue';
import InlineTradableEditor from '../tradables/InlineTradableEditor.vue';

const gameStore = useGameStore();
const usersStore = useUsersStore();
const modalStore = useModalStore();

const openAddStatModal = (playerId: string) => {
    modalStore.openModal(AddStatModal, { playerId });
};
</script>

<template>
    <div v-if="gameStore.game" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <fieldset
            v-for="player in gameStore.game.players"
            :key="player.id"
            class="fieldset rounded-box bg-base-200 p-4 border border-primary"
        >
            <legend class="fieldset-legend">{{ usersStore.getPlayerName(player.id) }}</legend>
            <h5 class="text-lg font-bold">Tradables</h5>
            <div class="flex flex-col gap-2">
                <InlineTradableEditor
                    v-for="tradable in player.tradables"
                    :key="tradable.id"
                    :playerId="player.id"
                    :tradable="tradable"
                />
            </div>
            <h5 class="text-lg font-bold">Stats</h5>
            <div class="flex flex-col gap-2">
                <InlineStatsEditor
                    v-for="stat in player.stats"
                    :key="stat.id"
                    :playerId="player.id"
                    :stat="stat"
                />
            </div>
            <button
                class="btn btn-sm btn-circle btn-primary mx-auto"
                title="Add Stat"
                @click="openAddStatModal(player.id)"
            >
                +
            </button>
        </fieldset>
    </div>
</template>
