<script setup lang="ts">
import { useGameEmitter } from '@/emitters/gameEmitter';

const gameEmitter = useGameEmitter();

function endGame() {
    if (confirm('Are you sure you want to end the game? This action cannot be undone.')) {
        gameEmitter.endGame();
    }
}
</script>

<template>
    <button class="btn btn-error btn-outline btn-sm w-full gap-2" @click="endGame">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
            ></path>
        </svg>
        End Game
    </button>
</template>
