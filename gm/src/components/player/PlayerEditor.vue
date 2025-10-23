<script setup lang="ts">
import { useAutosaveObjectEditor } from '@/composables/useAutosaveObjectEditor'
import PlayerStatsEditor from './PlayerStatsEditor.vue'
import { usePlayerStore } from '@/stores/playerStore'
import { usePlayerEmitter } from '@/emitters/playerEmitter'
import { onUnmounted } from 'vue'

const props = defineProps<{
    playerId: number
}>()

const emit = defineEmits(['done'])

const playerStore = usePlayerStore()
const playerEmitter = usePlayerEmitter()

const { editableObject, areEditableObjectFieldsChanged, handleFieldInput, saveChanges } =
    useAutosaveObjectEditor<{ name: string; secret: string }>(
        () => {
            const player = playerStore.getPlayerById(props.playerId)
            return {
                name: player?.name ?? '',
                secret: player?.secret ?? '',
            }
        },
        (newObject) => {
            playerEmitter.updatePlayer(props.playerId, {
                name: newObject.name,
                secret: newObject.secret,
            })
        },
    )

onUnmounted(() => {
    saveChanges()
})
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Edit Player</h1>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <label class="label"
                    >Player Name{{ areEditableObjectFieldsChanged.name ? '*' : '' }}</label
                >
                <input
                    type="text"
                    placeholder="Enter player name..."
                    :value="editableObject.name"
                    :class="
                        'input w-full ' +
                        (areEditableObjectFieldsChanged.name ? 'input-accent' : '')
                    "
                    @input="handleFieldInput('name', $event)"
                    @focusout="saveChanges"
                    @keypress="(e) => (e.key === 'Enter' ? saveChanges() : null)"
                />
            </div>

            <div>
                <label class="label"
                    >Secret Code{{ areEditableObjectFieldsChanged.secret ? '*' : '' }}</label
                >
                <input
                    type="text"
                    placeholder="Enter secret code..."
                    :value="editableObject.secret"
                    :class="
                        'input w-full ' +
                        (areEditableObjectFieldsChanged.secret ? 'input-accent' : '')
                    "
                    @input="handleFieldInput('secret', $event)"
                    @focusout="saveChanges"
                    @keypress="(e) => (e.key === 'Enter' ? saveChanges() : null)"
                />
            </div>
        </div>

        <PlayerStatsEditor
            :player-id="props.playerId"
            :player-name="playerStore.getPlayerById(props.playerId)!.name"
            :player-stats="playerStore.getPlayerById(props.playerId)!.stats"
        />

        <div class="divider"></div>

        <div class="card bg-error/5 border border-error/20">
            <div class="card-body">
                <p class="text-xs text-base-content/70 mb-4">
                    This action cannot be undone and will permanently remove the player from the
                    game.
                </p>
                <button
                    class="btn btn-error btn-sm gap-2"
                    @click="playerEmitter.deletePlayer(props.playerId)"
                >
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
