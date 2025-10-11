<script lang="ts" setup>
import { usePlayerStore } from '@/stores/playerStore'
import { useModalStore } from '@/stores/modalStore'
import PlayerEditorModal from '../player/PlayerEditorModal.vue'
import DisplayContainer from '../container/DisplayContainer.vue'

const playerStore = usePlayerStore()
const modalStore = useModalStore()

function openPlayerEditor(playerId: string) {
    modalStore.openModal(PlayerEditorModal, { playerId })
}
</script>

<template>
    <DisplayContainer label="Players">
        <div
            v-if="playerStore.players.length > 0"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
        >
            <div
                v-for="player in playerStore.players"
                :key="player.id"
                @click="openPlayerEditor(player.id)"
                class="card bg-base-100 border-2 border-primary/20 hover:border-primary hover:shadow-xl transition-all duration-200 cursor-pointer group"
            >
                <div class="card-body p-4">
                    <div class="flex items-start justify-between mb-3">
                        <h3 class="card-title text-lg text-primary group-hover:text-primary-focus">
                            {{ player.name }}
                        </h3>
                        <svg
                            class="w-5 h-5 text-base-content/50 group-hover:text-primary transition-colors"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                            ></path>
                        </svg>
                    </div>

                    <div class="mb-4">
                        <div class="badge badge-ghost badge-sm">Secret: {{ player.secret }}</div>
                    </div>

                    <div v-if="player.stats.length > 0" class="space-y-2">
                        <h4 class="text-sm font-semibold text-secondary mb-2">Stats</h4>
                        <div class="space-y-1">
                            <div
                                v-for="stat in player.stats.slice(0, 3)"
                                :key="stat.name"
                                class="flex justify-between items-center text-sm p-2 bg-base-200 rounded"
                            >
                                <span class="font-medium text-base-content/80">{{
                                    stat.name
                                }}</span>
                                <span class="badge badge-primary badge-sm">{{ stat.value }}</span>
                            </div>
                            <div
                                v-if="player.stats.length > 3"
                                class="text-xs text-base-content/60 text-center mt-2"
                            >
                                +{{ player.stats.length - 3 }} more stats
                            </div>
                        </div>
                    </div>

                    <div v-else class="text-sm text-base-content/60 italic">No stats defined</div>

                    <div class="card-actions justify-end mt-4">
                        <button class="btn btn-primary btn-xs">Edit Player</button>
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
    </DisplayContainer>
</template>
