<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore';
import { useModalStore } from '@/stores/modalStore';
import PlayerEditorModal from './PlayerEditorModal.vue';

const playerStore = usePlayerStore();
const modalStore = useModalStore();

function openPlayerEditor(playerId: number) {
    modalStore.openModal(PlayerEditorModal, { playerId }, '4xl');
}

function countLines(text: string | undefined): number {
    if (!text) return 0;
    return text.split('\n').length;
}

function getNotesMaxHeight(lines: number): string {
    // Scale height based on number of lines, cap at 5 lines
    const displayLines = Math.min(lines, 5);
    // Each line is approximately 1.5rem (24px with line-height), plus padding (1rem)
    const height = displayLines * 1.5 + 1;
    return `${height}rem`;
}

function getStatsMaxHeight(statsCount: number): string | undefined {
    // Only apply max height if more than 10 stats
    if (statsCount <= 10) return undefined;
    // Each stat is approximately 2rem in height (including gap), cap at 10 visible
    const height = 10 * 2;
    return `${height}rem`;
}
</script>

<template>
    <div>
        <div v-if="playerStore.players.length > 0" class="flex flex-wrap gap-3">
            <div
                v-for="player in playerStore.players"
                :key="player.id"
                @click="openPlayerEditor(player.id)"
                class="card bg-base-200 border border-base-300 w-[calc(25%-0.75rem)] cursor-pointer hover:border-primary hover:shadow-lg transition-all duration-200"
            >
                <div class="card-body p-3">
                    <!-- Player Header -->
                    <h2 class="text-lg font-bold text-primary mb-2">
                        {{ player.name }}
                        <span class="badge badge-xs badge-primary justify-self-start">{{
                            player.secret
                        }}</span>
                    </h2>

                    <div class="flex gap-3">
                        <!-- Stats Column -->
                        <div class="flex-1">
                            <h3 class="text-xs font-semibold text-secondary mb-1">Stats</h3>
                            <div
                                v-if="player.stats.length > 0"
                                :style="{
                                    maxHeight: getStatsMaxHeight(player.stats.length),
                                }"
                                class="space-y-1 overflow-y-auto"
                            >
                                <div
                                    v-for="stat in player.stats"
                                    :key="stat.id"
                                    class="grid grid-cols-[max-content_1px_1fr] gap-2 items-center text-xs p-1.5 bg-base-100 rounded"
                                >
                                    <span class="font-medium">{{ stat.name }}</span>
                                    <div class="h-4 w-px bg-base-content/20"></div>
                                    <span class="justify-self-start italic">{{ stat.value }}</span>
                                </div>
                            </div>
                            <div v-else class="text-xs text-base-content/60 italic">
                                No stats defined
                            </div>
                        </div>

                        <!-- Notes Column -->
                        <div class="flex-1 space-y-2">
                            <!-- Notes -->
                            <div>
                                <h3 class="text-xs font-semibold text-secondary mb-1">Notes</h3>
                                <div
                                    v-if="player.notes"
                                    :style="{
                                        maxHeight: getNotesMaxHeight(countLines(player.notes)),
                                    }"
                                    class="text-xs p-2 bg-base-100 rounded whitespace-pre-wrap overflow-y-auto"
                                >
                                    {{ player.notes }}
                                </div>
                                <div v-else class="text-xs text-base-content/60 italic">
                                    No notes
                                </div>
                            </div>

                            <!-- Hidden Notes -->
                            <div>
                                <h3 class="text-xs font-semibold text-warning mb-1">
                                    Hidden Notes
                                </h3>
                                <div
                                    v-if="player.hiddenNotes"
                                    :style="{
                                        maxHeight: getNotesMaxHeight(
                                            countLines(player.hiddenNotes),
                                        ),
                                    }"
                                    class="text-xs p-2 bg-warning/10 border border-warning/20 rounded whitespace-pre-wrap overflow-y-auto"
                                >
                                    {{ player.hiddenNotes }}
                                </div>
                                <div v-else class="text-xs text-base-content/60 italic">
                                    No hidden notes
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div v-else class="text-center py-12">
            <div class="flex flex-col items-center space-y-4">
                <svg
                    class="w-16 h-16 text-base-content/20"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                    ></path>
                </svg>
                <div>
                    <h3 class="text-lg font-semibold text-base-content/70">No Players Yet</h3>
                    <p class="text-sm text-base-content/50 mt-1">
                        Use the sidebar to create your first player
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>
