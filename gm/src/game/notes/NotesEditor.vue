<script setup lang="ts">
import { ref } from 'vue';
import { useEventEmitter } from '../../events/useEventEmitter';
import { type Game } from '../gameStore';

const props = defineProps<{
    game: Game;
}>();

const eventEmitter = useEventEmitter();

const notes = ref<string>(props.game.notes);
const hiddenNotes = ref<string>(props.game.hiddenNotes);
</script>

<template>
    <!-- use a daisyUI card container for visual grouping -->
    <div class="w-full">
        <div class="flex flex-col gap-2">
            <h2 class="card-title">Game Notes</h2>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Game Notes</legend>
                <textarea
                    v-model="notes"
                    class="textarea textarea-bordered h-[40vh] w-full"
                    placeholder="Enter your notes here..."
                    @change="() => eventEmitter.setNotes(notes)"
                ></textarea>
            </fieldset>

            <fieldset class="fieldset">
                <legend class="fieldset-legend">Hidden Game Notes</legend>
                <textarea
                    v-model="hiddenNotes"
                    class="textarea textarea-bordered h-[30vh] w-full"
                    placeholder="Enter your hidden notes here..."
                    @change="() => eventEmitter.setHiddenNotes(hiddenNotes)"
                ></textarea>
            </fieldset>
        </div>
    </div>
</template>
