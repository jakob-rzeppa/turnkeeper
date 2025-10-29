<script setup lang="ts">
import useConnection from '@/composables/useConnection';
import { ref } from 'vue';

const connection = useConnection();

const name = ref('');
const secret = ref('');

// Try to connect with existing cookie on component mount
connection.connectWithCookie();

function login() {
    connection.connect({ playerName: name.value, playerSecret: secret.value });
}
</script>

<template>
    <div class="min-h-screen flex items-center justify-center p-4">
        <div class="card bg-base-100 shadow-xl w-full max-w-md">
            <div class="card-body">
                <h1 class="card-title text-2xl font-bold text-center mb-6">Turnkeeper Login</h1>

                <div class="w-full">
                    <label class="label">
                        <span class="label-text">Player Name</span>
                    </label>
                    <input
                        v-model="name"
                        type="text"
                        placeholder="Enter your name"
                        class="input input-bordered w-full"
                    />
                </div>

                <div class="w-full">
                    <label class="label">
                        <span class="label-text">Player Secret</span>
                    </label>
                    <input
                        v-model="secret"
                        type="password"
                        placeholder="Enter your secret"
                        class="input input-bordered w-full"
                    />
                </div>

                <div class="card-actions justify-center mt-6">
                    <button
                        @click="login"
                        class="btn btn-primary btn-wide"
                        :disabled="!name || !secret"
                    >
                        Join Game
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
