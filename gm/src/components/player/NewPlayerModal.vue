<script setup lang="ts">
import { usePlayerEmitter } from '@/emitters/playerEmitter'
import { ref } from 'vue'

const emit = defineEmits(['close'])

const playerEmitter = usePlayerEmitter()

const newPlayerNameRef = ref('')

function createPlayer() {
    playerEmitter.createPlayer(newPlayerNameRef.value)
    emit('close')
}
</script>

<template>
    <div class="space-y-6">
        <div class="text-center">
            <h1 class="text-3xl font-bold text-primary mb-2">Create New Player</h1>
            <p class="text-base-content/70">Add a new player to your game session</p>
        </div>

        <div class="form-control">
            <label class="label">
                <span class="label-text font-medium">Player Name</span>
            </label>
            <input
                type="text"
                v-model="newPlayerNameRef"
                placeholder="Enter player name..."
                class="input input-bordered input-primary w-full"
                @keyup.enter="createPlayer"
            />
        </div>

        <button
            class="btn btn-primary btn-lg w-full"
            @click="createPlayer"
            :disabled="!newPlayerNameRef.trim()"
        >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                ></path>
            </svg>
            Create Player
        </button>
    </div>
</template>
