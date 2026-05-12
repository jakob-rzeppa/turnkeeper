<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useGameEditor } from './useGameEditor';

const route = useRoute();

const gameEditor = useGameEditor();

function handleKeyDown(event: KeyboardEvent) {
    // Handle Cmd+S / Ctrl+S for save
    if ((event.metaKey || event.ctrlKey) && event.key === 's') {
        event.preventDefault();
        gameEditor.saveSourceCode(route.params.id as string);
        return;
    }

    if (event.key === 'Tab') {
        event.preventDefault();
        const textarea = event.target as HTMLTextAreaElement;
        const start = textarea.selectionStart;
        const end = textarea.selectionEnd;

        // Calculate column position in the current line
        const textBeforeCursor = gameEditor.sourceCode.value.substring(0, start);
        const lastNewlineIndex = textBeforeCursor.lastIndexOf('\n');
        const columnInLine = lastNewlineIndex === -1 ? start : start - lastNewlineIndex - 1;

        // Calculate spaces needed to reach next 4-space tab stop
        const spacesToTab = 4 - (columnInLine % 4);

        // Insert spaces to snap to tab point
        gameEditor.sourceCode.value =
            gameEditor.sourceCode.value.substring(0, start) +
            ' '.repeat(spacesToTab) +
            gameEditor.sourceCode.value.substring(end);

        // Move cursor after the inserted spaces
        setTimeout(() => {
            textarea.selectionStart = textarea.selectionEnd = start + spacesToTab;
        }, 0);
    }
}
</script>

<template>
    <div class="w-full h-full">
        <textarea
            class="w-full h-full input"
            :class="{ 'input-warning': !gameEditor.isSaved.value }"
            v-model="gameEditor.sourceCode.value"
            @keydown="handleKeyDown"
        ></textarea>
    </div>
</template>
