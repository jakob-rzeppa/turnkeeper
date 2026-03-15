<script setup lang="ts">
import type { Player } from '../gameStore';
import InlineTradableEditor from '../tradables/InlineTradableEditor.vue';

const props = defineProps<{
    player: Player;
}>();
</script>

<template>
    <div class="space-y-4">
        <div v-if="player.tradables.length === 0" class="alert alert-info">
            <span>No tradables for this player.</span>
        </div>
        <div v-else class="space-y-2">
            <InlineTradableEditor
                v-for="tradable in player.tradables"
                :key="tradable.id"
                :playerId="props.player.id"
                :tradable="tradable"
                size="lg"
            />
        </div>
    </div>
</template>
