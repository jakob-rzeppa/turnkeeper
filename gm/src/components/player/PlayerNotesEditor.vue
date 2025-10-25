<script setup lang="ts">
import { useAutosaveObject } from '@/composables/useAutosaveObject';
import { usePlayerEmitter } from '@/emitters/playerEmitter';
import { usePlayerStore } from '@/stores/playerStore';
import { onUnmounted } from 'vue';

const props = defineProps<{
    playerId: number;
}>();

const playerStore = usePlayerStore();
const playerEmitter = usePlayerEmitter();

const { editableObject, idEditableObjectChanged, saveChanges } = useAutosaveObject<{
    name: string;
    secret: string;
    notes: string;
}>(
    () => {
        const player = playerStore.getPlayerById(props.playerId);

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
</template>
