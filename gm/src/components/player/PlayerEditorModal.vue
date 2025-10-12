<script setup lang="ts">
import PlayerEditor from './PlayerEditor.vue'
import { usePlayerEmitter } from '@/emitters/playerEmitter'

// The Player prop needs to be a deep clone
const props = defineProps<{
    playerId: string
}>()

const emit = defineEmits(['close'])

const playerEmitter = usePlayerEmitter()

function deletePlayer(): void {
    if (
        confirm(
            `Are you sure you want to delete player with id ${props.playerId}? This action cannot be undone.`,
        )
    ) {
        playerEmitter.deletePlayer(props.playerId)
        emit('close')
    }
}
</script>

<template>
    <div class="space-y-6">
        <PlayerEditor :playerId="props.playerId" @done="emit('close')" />

        <div class="divider"></div>

        <div class="card bg-error/5 border border-error/20">
            <div class="card-body">
                <p class="text-xs text-base-content/70 mb-4">
                    This action cannot be undone and will permanently remove the player from the
                    game.
                </p>
                <button class="btn btn-error btn-sm gap-2" @click="deletePlayer">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                        ></path>
                    </svg>
                    Delete Player
                </button>
            </div>
        </div>
    </div>
</template>
