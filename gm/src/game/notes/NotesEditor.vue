<script setup lang="ts">
import { useAutosaveObject } from '../../common/autosaveObject';
import { useEventEmitter } from '../../events/useEventEmitter';
import { type Game } from '../gameStore';

const props = defineProps<{
    game: Game;
}>();

const eventEmitter = useEventEmitter();

const autosave = useAutosaveObject<{
    notes: string;
    hiddenNotes: string;
}>(
    () => {
        return {
            notes: props.game.notes,
            hiddenNotes: props.game.hiddenNotes,
        };
    },
    newObject => {
        eventEmitter.emit({
            SetNotes: newObject.notes,
        });
        eventEmitter.emit({
            SetHiddenNotes: newObject.hiddenNotes,
        });
    }
);
</script>

<template>
    <!-- use a daisyUI card container for visual grouping -->
    <div class="w-full" @focusout="autosave.saveChanges">
        <div class="flex flex-col gap-2">
            <h2 class="card-title">
                {{ `Game Notes${autosave.isEditableObjectChanged.value ? '*' : ''}` }}
            </h2>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Game Notes</legend>
                <textarea
                    v-model="autosave.editableObject.value.notes"
                    class="textarea textarea-bordered h-[40vh] w-full"
                    placeholder="Enter your notes here..."
                ></textarea>
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Hidden Game Notes</legend>
                <textarea
                    v-model="autosave.editableObject.value.hiddenNotes"
                    class="textarea textarea-bordered h-[30vh] w-full"
                    placeholder="Enter your hidden notes here..."
                ></textarea>
            </fieldset>
        </div>
    </div>
</template>
