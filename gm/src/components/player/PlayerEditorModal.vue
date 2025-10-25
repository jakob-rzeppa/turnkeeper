<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore';
import { usePlayerEmitter } from '@/emitters/playerEmitter';
import { useAutosaveObject } from '@/composables/useAutosaveObject';
import { onUnmounted } from 'vue';
import PlayerStatsEditor from './PlayerStatsEditor.vue';

// The Player prop needs to be a deep clone
const props = defineProps<{
    playerId: number;
}>();

const emit = defineEmits(['close']);

const playerStore = usePlayerStore();
const playerEmitter = usePlayerEmitter();

const { editableObject, idEditableObjectChanged, saveChanges } = useAutosaveObject<{
    name: string;
    secret: string;
    notes: string;
}>(
    () => {
        const player = playerStore.getPlayerById(props.playerId);

        // If no player is found, close the editor (e.g. player was deleted)
        if (!player) emit('close');

        return {
            name: player?.name ?? '',
            secret: player?.secret ?? '',
            notes: player?.notes ?? '',
        };
    },
    (newObject) => {
        playerEmitter.updatePlayer(props.playerId, {
            name: newObject.name,
            secret: newObject.secret,
            notes: newObject.notes,
        });
    },
);

onUnmounted(() => {
    saveChanges();
});
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Edit Player</h1>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <label class="label">Player Name{{ idEditableObjectChanged ? '*' : '' }}</label>
                <input
                    type="text"
                    placeholder="Enter player name..."
                    v-model="editableObject.name"
                    :class="'input w-full ' + (idEditableObjectChanged ? 'input-accent' : '')"
                    @focusout="saveChanges"
                    @keypress="(e) => (e.key === 'Enter' ? saveChanges() : null)"
                />
            </div>

            <div>
                <label class="label">Secret Code{{ idEditableObjectChanged ? '*' : '' }}</label>
                <input
                    type="text"
                    placeholder="Enter secret code..."
                    v-model="editableObject.secret"
                    :class="'input w-full ' + (idEditableObjectChanged ? 'input-accent' : '')"
                    @focusout="saveChanges"
                    @keypress="(e) => (e.key === 'Enter' ? saveChanges() : null)"
                />
            </div>
        </div>

        <div>
            <label class="label">Notes{{ idEditableObjectChanged ? '*' : '' }}</label>
            <textarea
                class="textarea w-full h-32"
                :class="idEditableObjectChanged ? 'textarea-accent' : ''"
                placeholder="Enter notes about the player..."
                v-model="editableObject.notes"
                @focusout="saveChanges"
            ></textarea>
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
                    @click="() => playerEmitter.deletePlayer(props.playerId)"
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
