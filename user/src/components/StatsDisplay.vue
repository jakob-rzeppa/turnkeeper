<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore';

const playerStore = usePlayerStore();
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

        <div v-else class="overflow-x-auto">
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th class="w-1">Stat</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="stat in playerStore.player.stats" :key="stat.name">
                        <td class="font-semibold whitespace-nowrap">{{ stat.name }}</td>
                        <td>
                            <div v-if="typeof stat.value === 'boolean'" class="flex">
                                <div
                                    :class="['badge', stat.value ? 'badge-success' : 'badge-error']"
                                >
                                    {{ stat.value ? 'True' : 'False' }}
                                </div>
                            </div>
                            <div
                                v-else-if="typeof stat.value === 'number'"
                                class="font-semibold italic"
                            >
                                {{ stat.value }}
                            </div>
                            <div v-else class="font-semibold">
                                {{ stat.value }}
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>
