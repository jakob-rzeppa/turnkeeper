<script setup lang="ts">
import { useGameStore } from '../gameStore';
import NotesEditor from '../notes/NotesEditor.vue';

const gameStore = useGameStore();
</script>

<template>
    <NotesEditor v-if="gameStore.game" :game="gameStore.game" />
</template>
