<script setup lang="ts">
import { useSession } from '../useSession';
import type { StatValue } from '../types/state';

const session = useSession();

const getStatValue = (stat: StatValue): string | number | boolean => {
    if (stat.int_value !== null) return stat.int_value;
    if (stat.float_value !== null) return stat.float_value;
    if (stat.str_value !== null) return stat.str_value;
    if (stat.bool_value !== null) return stat.bool_value;
    return 'N/A';
};
</script>

<template>
    <div v-if="session.connectionStatus.value !== 'connected'">
        <div class="h-full flex items-center justify-center">
            <span class="loading loading-lg"></span>
        </div>
    </div>
    <div v-else class="p-4 space-y-6">
        <!-- Game Info -->
        <div v-if="session.gameState.value">
            <div class="text-sm text-gray-500 mb-2">
                Round <span class="font-bold text-lg">{{ session.gameState.value.round }}</span>
            </div>
        </div>

        <!-- Game Stats -->
        <div>
            <h2 class="text-2xl font-bold mb-4">Game Stats</h2>
            <div v-if="session.gameState.value && session.gameState.value.game_stats.length > 0" class="space-y-3">
                <div
                    v-for="stat in session.gameState.value.game_stats"
                    :key="stat.name"
                    class="p-3 bg-base-200 rounded"
                >
                    <div class="font-semibold mb-2">{{ stat.name }}</div>
                    <div class="flex gap-6 text-sm">
                        <div>
                            <span class="text-gray-600">Value:</span>
                            <span class="font-semibold ml-2">{{ getStatValue(stat.value) }}</span>
                        </div>
                        <div>
                            <span class="text-gray-600">Default:</span>
                            <span class="font-semibold ml-2">{{ getStatValue(stat.default) }}</span>
                        </div>
                    </div>
                </div>
            </div>
            <p v-else class="text-gray-500 italic">No game stats available</p>
        </div>

        <!-- Players -->
        <div>
            <h2 class="text-2xl font-bold mb-4">Players</h2>
            <div v-if="session.gameState.value && session.gameState.value.players.length > 0" class="space-y-4">
                <div
                    v-for="(player, index) in session.gameState.value.players"
                    :key="player.name"
                    class="p-4 bg-base-200 rounded"
                >
                    <div class="flex items-center mb-3">
                        <span class="font-bold text-lg">{{ player.name }}</span>
                        <span v-if="index === session.gameState.value?.current_player_index" class="ml-2 badge badge-primary">
                            Current Turn
                        </span>
                    </div>
                    <div v-if="session.gameState.value.player_stats.length > 0" class="space-y-2 ml-2">
                        <div v-for="playerStat in session.gameState.value.player_stats" :key="playerStat.name" class="text-sm">
                            <span class="text-gray-600">{{ playerStat.name }}:</span>
                            <span class="font-semibold ml-2">
                                {{
                                    playerStat.values.find(([name]) => name === player.name)?.[1]
                                        ? getStatValue(playerStat.values.find(([name]) => name === player.name)![1])
                                        : 'N/A'
                                }}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <p v-else class="text-gray-500 italic">No players available</p>
        </div>

        <!-- Actions -->
        <div>
            <h2 class="text-2xl font-bold mb-4">Available Actions</h2>
            <div v-if="session.displayTemplate.value && session.displayTemplate.value.actions.length > 0" class="space-y-3">
                <div
                    v-for="action in session.displayTemplate.value.actions"
                    :key="action.name"
                    class="p-3 bg-base-200 rounded"
                >
                    <div class="font-semibold mb-2">{{ action.name }}</div>
                    <div class="space-y-2 text-sm">
                        <div v-if="action.parameters.length > 0" class="text-gray-600">
                            <span>Parameters:</span>
                            <div class="font-mono text-gray-700 mt-1">{{ action.parameters.join(', ') }}</div>
                        </div>
                        <div v-if="action.execution_triggers.length > 0" class="text-gray-600">
                            <span>Triggers:</span>
                            <div class="font-mono text-gray-700 mt-1">{{ action.execution_triggers.join(', ') }}</div>
                        </div>
                    </div>
                </div>
            </div>
            <p v-else class="text-gray-500 italic">No actions available</p>
        </div>
    </div>
</template>