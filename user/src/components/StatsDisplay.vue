<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'

const playerStore = usePlayerStore()
</script>

<template>
    <div class="divider">Stats</div>

    <div v-if="!playerStore.player">
        <div class="alert alert-error">
            <span>No player found</span>
        </div>
    </div>

    <div v-else>
        <div
            v-if="!(playerStore.player.stats && playerStore.player.stats.length > 0)"
            class="text-center py-8"
        >
            <div class="text-base-content/60">
                <p>No stats found</p>
            </div>
        </div>

        <div v-else class="space-y-3">
            <div
                v-for="stat in playerStore.player.stats"
                :key="stat.name"
                class="flex items-center justify-between p-4 bg-base-200 rounded-lg"
            >
                <div class="border-r border-base-300 mr-3 pr-3">
                    <span class="font-semibold w-fit">{{ stat.name }}</span>
                </div>

                <div>
                    <div v-if="typeof stat.value === 'boolean'" class="flex items-center">
                        <div :class="['badge', stat.value ? 'badge-success' : 'badge-error']">
                            {{ stat.value ? 'Yes' : 'No' }}
                        </div>
                    </div>
                    <div
                        v-else-if="typeof stat.value === 'number'"
                        class="text-2xl font-bold text-primary"
                    >
                        {{ stat.value }}
                    </div>
                    <div
                        v-else-if="Array.isArray(stat.value)"
                        class="flex flex-wrap gap-1 justify-end"
                    >
                        <div
                            v-for="item in stat.value"
                            :key="item"
                            class="badge badge-outline badge-sm"
                        >
                            {{ item }}
                        </div>
                    </div>
                    <div v-else class="text-lg font-semibold">
                        {{ stat.value }}
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
