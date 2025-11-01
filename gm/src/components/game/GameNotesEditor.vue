<script setup lang="ts">
import { useGameStore } from '@/stores/gameStore';
import DisplayContainer from '../container/DisplayContainer.vue';
import { useAutosaveObject } from '@/composables/useAutosaveObject';
import { onUnmounted } from 'vue';
import { useGameEmitter } from '@/emitters/gameEmitter';

const gameStore = useGameStore();

const gameEmitter = useGameEmitter();

const autosave = useAutosaveObject<{
    notes: string;
    hiddenNotes: string;
}>(
    () => {
        return {
            notes: gameStore.notes,
            hiddenNotes: gameStore.hiddenNotes,
        };
    },
    (newObject) => {
        gameEmitter.updateGameNotes(newObject.notes);
        gameEmitter.updateHiddenNotes(newObject.hiddenNotes);
    },
);

onUnmounted(() => {
    autosave.saveChanges();
});
</script>

<template>
    <DisplayContainer :label="'Game Notes' + (autosave.isEditableObjectChanged.value ? '*' : '')">
        <div
            class="flex flex-col gap-2"
            @focusout="autosave.saveChanges"
            v-if="gameStore.isInitialized"
        >
            <div>
                <label for="game-notes" class="font-semibold">Game Notes</label>
                <textarea
                    id="game-notes"
                    v-model="autosave.editableObject.value.notes"
                    class="textarea textarea-bordered w-full min-h-32"
                    :class="autosave.isEditableObjectChanged.value ? 'textarea-accent' : ''"
                    placeholder="Enter game notes here..."
                ></textarea>
            </div>
            <div>
                <label for="game-hidden-notes" class="font-semibold">Game Hidden Notes</label>
                <textarea
                    id="game-hidden-notes"
                    v-model="autosave.editableObject.value.hiddenNotes"
                    class="textarea textarea-bordered w-full min-h-32"
                    :class="autosave.isEditableObjectChanged.value ? 'textarea-accent' : ''"
                    placeholder="Enter game hidden notes here..."
                ></textarea>
            </div>
        </div>
        <div v-else class="text-center py-8">
            <div class="alert alert-info">
                <svg class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                </svg>
                <span>Game not initialized. Use the drawer to start a new game.</span>
            </div>
        </div>
    </DisplayContainer>
</template>
