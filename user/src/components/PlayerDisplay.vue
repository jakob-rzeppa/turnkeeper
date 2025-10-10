<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore'

const playerStore = usePlayerStore()
</script>

<template>
    <div v-if="!playerStore.player">No player found</div>
    <div v-else>
        <h2>{{ playerStore.player.name }}</h2>
        <ul class="list">
            <li v-for="stat in playerStore.player.stats" :key="stat.name">
                {{ stat.name }}: {{ stat.value }}
            </li>
        </ul>
    </div>
</template>
