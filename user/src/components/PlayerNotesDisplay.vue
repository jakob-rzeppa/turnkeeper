<script setup lang="ts">
import { usePlayerStore } from '@/stores/playerStore';

const playerStore = usePlayerStore();
</script>

<template>
    <div class="divider">Notes</div>
    <div v-if="!playerStore.player">
        <div class="alert alert-error">
            <span>No player found</span>
        </div>
    </div>
    <div v-else>
        <div class="p-4 bg-base-200 rounded-lg">
            <pre class="whitespace-pre-wrap break-words text-sm">{{
                playerStore.player.notes
            }}</pre>
        </div>
    </div>
</template>
