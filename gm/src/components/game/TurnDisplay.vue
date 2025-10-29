<script setup lang="ts">
import { useGameEmitter } from '@/emitters/gameEmitter';
import DisplayContainer from '../container/DisplayContainer.vue';
import { useGameStore } from '@/stores/gameStore';
import { useModalStore } from '@/stores/modalStore';
import PlayerEditorModal from '../player/PlayerEditorModal.vue';

const gameStore = useGameStore();
const modalStore = useModalStore();

const gameEmitter = useGameEmitter();

function endTurn() {
    gameEmitter.nextTurn();
}
</script>

<template>
    <DisplayContainer label="Turn Management">
        <div v-if="gameStore.isInitialized" class="flex flex-col gap-4">
            <div class="flex flex-row align-middle gap-4">
                <!-- Round Info -->
                <div class="stats shadow-lg w-max">
                    <div class="stat">
                        <div class="stat-title">Current Round</div>
                        <div class="stat-value text-primary">{{ gameStore.round.roundNumber }}</div>
                    </div>
                </div>

                <!-- End Turn Button -->
                <button class="btn btn-accent btn-lg flex-1 my-auto" @click="endTurn">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 5l7 7-7 7"
                        ></path>
                    </svg>
                    End Turn
                </button>
            </div>

            <!-- Player Order Breadcrumbs -->
            <div class="breadcrumbs">
                <ul class="text-sm">
                    <li
                        v-for="(player, index) in gameStore.playerOrder"
                        :key="player.id"
                        class="flex items-center"
                    >
                        <div
                            class="flex items-center space-x-2"
                            :class="{
                                'hover:cursor-pointer hover:bg-accent hover:text-accent-content pl-2 pr-4 py-2 rounded-full ': true,
                                'bg-accent text-accent-content font-bold hover:bg-secondary hover:text-secondary-content':
                                    gameStore.currentPlayerId === player.id,
                                'text-base-content/70': gameStore.currentPlayerId !== player.id,
                            }"
                            @click="
                                modalStore.openModal(
                                    PlayerEditorModal,
                                    { playerId: player.id },
                                    '4xl',
                                )
                            "
                        >
                            <span
                                class="inline-flex items-center justify-center w-6 h-6 text-xs rounded-full bg-primary text-primary-content"
                            >
                                {{ index + 1 }}
                            </span>
                            <span>{{ player.name }}</span>
                        </div>
                    </li>
                </ul>
            </div>
        </div>
        <div v-else class="text-center py-8">
            <div class="alert alert-info">
                <svg class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                </svg>
                <span>Game not initialized. Use the drawer to start a new game.</span>
            </div>
        </div>
    </DisplayContainer>
</template>
