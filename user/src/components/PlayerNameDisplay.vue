<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore';

const { player } = usePlayerStore();
</script>

<template>
    <h1 v-if="player" class="text-3xl text-primary text-center font-bold">{{ player.name }}</h1>
</template>
