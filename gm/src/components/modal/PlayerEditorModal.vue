<script setup lang="ts">
const props = defineProps<{
    player: { name: string; secret: string; stats: string[] }
}>()
</script>

<template>
    <h2>Edit Player: {{ props.player.name }}</h2>
    <p>Secret: {{ props.player.secret }}</p>
    <ul>
        <li v-for="(stat, index) in props.player.stats" :key="index">
            {{ stat }}
        </li>
    </ul>
</template>
