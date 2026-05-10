<script setup lang="ts">
import { useSession } from '../useSession';
import StatDisplay from '../components/StatDisplay.vue';
import ActionExecutor from '../components/ActionExecutor.vue';

const session = useSession();
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
                    class="p-3 bg-base-200 rounded flex flex-col gap-4"
                >
                    <StatDisplay 
                        v-for="stat in session.gameState.value.game_stats"
                        :key="stat.name" 
                        :statName="stat.name" 
                        :player="null" 
                        :editable="true"
                    />
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
                        <StatDisplay 
                            v-for="playerStat in session.gameState.value.player_stats"
                            :key="playerStat.name" 
                            :statName="playerStat.name" 
                            :player="player.name" 
                            :editable="true"
                        />
                    </div>
                </div>
            </div>
            <p v-else class="text-gray-500 italic">No players available</p>
        </div>

        <!-- Actions -->
        <div>
            <h2 class="text-2xl font-bold mb-4">Available Actions</h2>
            <div v-if="session.displayTemplate.value && session.displayTemplate.value.actions.length > 0" class="space-y-3 p-3 bg-base-200 rounded">
                <ActionExecutor 
                    v-for="action in session.displayTemplate.value.actions" 
                    :key="action.name" 
                    :actionName="action.name"
                />
            </div>
            <p v-else class="text-gray-500 italic">No actions available</p>
        </div>
    </div>
</template>