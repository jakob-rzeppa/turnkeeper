<script setup lang="ts">
import { useWsConnection } from '../api/useWsConnection';
import { useUsersStore } from '../users/usersStore';
import { useGameStore } from './gameStore';

const gameStore = useGameStore();
const usersStore = useUsersStore();
const wsConnection = useWsConnection();
</script>

<template>
    <div v-if="gameStore.game" class="min-h-screen bg-base-100 p-4 md:p-8">
        <div class="max-w-6xl mx-auto">
            <!-- Game Header -->
            <div class="mb-6 md:mb-8">
                <h1 class="text-3xl md:text-5xl font-bold text-base-content mb-2">
                    {{ gameStore.game.name }}
                </h1>
            </div>

            <!-- Game Status Bar -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-3 md:gap-4 mb-6 md:mb-8">
                <div class="bg-base-200 rounded-lg p-4 md:p-6 border border-base-300">
                    <p class="text-base-content/60 text-xs md:text-sm font-semibold uppercase">
                        Round
                    </p>
                    <p class="text-3xl md:text-4xl font-bold text-info">
                        {{ gameStore.game.roundNumber }}
                    </p>
                </div>
                <div class="bg-base-200 rounded-lg p-4 md:p-6 border border-base-300">
                    <p class="text-base-content/60 text-xs md:text-sm font-semibold uppercase">
                        Total Players
                    </p>
                    <p class="text-3xl md:text-4xl font-bold text-success">
                        {{ gameStore.game.players.length }}
                    </p>
                </div>
                <div class="bg-base-200 rounded-lg p-4 md:p-6 border border-base-300">
                    <p class="text-base-content/60 text-xs md:text-sm font-semibold uppercase">
                        Current Turn
                    </p>
                    <p class="text-3xl md:text-4xl font-bold text-secondary">
                        {{
                            usersStore.getPlayerName(
                                gameStore.game.players[gameStore.game.currentPlayerIndex]?.id || ''
                            )
                        }}
                    </p>
                </div>
            </div>

            <!-- Your Player Info -->
            <div class="bg-base-200 rounded-lg p-4 md:p-6 mb-6 md:mb-8 border border-base-300">
                <h2 class="text-xl md:text-2xl font-bold text-base-content mb-4 md:mb-6">
                    You ({{ usersStore.getPlayerName(gameStore.game.ownPlayer?.id || '') }})
                </h2>
                <div v-if="gameStore.game.ownPlayer">
                    <div class="mb-6">
                        <p
                            class="text-base-content/60 text-xs md:text-sm font-semibold uppercase mb-3 md:mb-4"
                        >
                            Stats
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-4">
                            <div
                                v-for="stat in gameStore.game.ownPlayer.stats"
                                :key="stat.id"
                                class="bg-base-300 rounded p-3 md:p-4 border border-base-300"
                            >
                                <p class="text-base-content/70 text-xs md:text-sm mb-2">
                                    {{ stat.key }}
                                </p>
                                <p
                                    v-if="stat.valueType === 'number'"
                                    class="text-xl md:text-2xl font-bold text-primary"
                                >
                                    {{ stat.numberValue }}
                                </p>
                                <p
                                    v-else-if="stat.valueType === 'string'"
                                    class="text-base md:text-lg text-primary font-semibold"
                                >
                                    {{ stat.stringValue }}
                                </p>
                                <p
                                    v-else-if="stat.valueType === 'boolean'"
                                    class="text-lg font-semibold"
                                    :class="stat.booleanValue ? 'text-success' : 'text-error'"
                                >
                                    {{ stat.booleanValue ? 'Yes' : 'No' }}
                                </p>
                            </div>
                        </div>
                    </div>

                    <!-- Tradables -->
                    <div v-if="gameStore.game.ownPlayer.tradables.length > 0">
                        <p
                            class="text-base-content/60 text-xs md:text-sm font-semibold uppercase mb-3 md:mb-4"
                        >
                            Tradables
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-4">
                            <div
                                v-for="tradable in gameStore.game.ownPlayer.tradables"
                                :key="tradable.id"
                                class="bg-base-300 rounded p-3 md:p-4 border border-warning border-l-4"
                            >
                                <p class="text-base-content/70 text-xs md:text-sm mb-2">
                                    {{ tradable.name }}
                                </p>
                                <p class="text-xl md:text-2xl font-bold text-warning">
                                    {{ tradable.value }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                <div v-else class="text-base-content/60">
                    <p class="text-lg">You have no player assigned to this game yet.</p>
                </div>
            </div>

            <!-- Game Notes -->
            <div class="bg-base-200 rounded-lg p-4 md:p-6 mb-6 md:mb-8 border border-base-300">
                <h2 class="text-xl md:text-2xl font-bold text-base-content mb-3 md:mb-4">
                    Game Notes
                </h2>
                <pre
                    class="bg-base-300 rounded p-3 md:p-4 text-base-content overflow-x-auto whitespace-pre-wrap wrap-break-words text-xs md:text-sm"
                    >{{ gameStore.game.notes }}</pre
                >
            </div>

            <!-- All Players -->
            <div class="bg-base-200 rounded-lg p-4 md:p-6 mb-6 md:mb-8 border border-base-300">
                <h2 class="text-xl md:text-2xl font-bold text-base-content mb-4 md:mb-6">
                    Players ({{ gameStore.game.players.length }})
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-4">
                    <div
                        v-for="(player, index) in gameStore.game.players"
                        :key="player.id"
                        class="bg-base-300 rounded p-3 md:p-4 border border-base-300"
                        :class="{
                            'border-warning border-2': index === gameStore.game.currentPlayerIndex,
                        }"
                    >
                        <p
                            class="text-base-content/60 text-xs md:text-sm font-semibold uppercase mb-2"
                        >
                            {{ usersStore.getPlayerName(player.id) }}
                            {{ player.id === gameStore.game.ownPlayer?.id ? '(You)' : '' }}
                        </p>
                        <div
                            v-if="index === gameStore.game.currentPlayerIndex"
                            class="mt-3 pt-3 border-t border-base-300"
                        >
                            <span
                                class="inline-block bg-warning text-base-100 px-2 md:px-3 py-1 rounded text-xs md:text-sm font-semibold"
                            >
                                Current Turn
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <button @click="wsConnection.disconnect()" class="btn btn-error">Leave Game</button>
        </div>
    </div>
    <div v-else>
        <h1 class="text-2xl md:text-4xl font-bold text-center text-base-content py-8">
            Waiting for game...
        </h1>
    </div>
</template>
