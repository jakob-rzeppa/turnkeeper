<script setup lang="ts">
import { ref } from 'vue';
import type { GameMetadata } from '../types/game';
import { request } from '../api/httpApi';

const response = ref(request<{ games: GameMetadata[] }>('GET', '/games'));
</script>

<template>
    <div v-if="response.loading" class="d-flex justify-content-center align-items-center min-vh-50">
        <div class="spinner-border text-primary" role="status">
            <span class="visually-hidden">Loading games...</span>
        </div>
    </div>
    <div v-else-if="response.error" class="alert alert-danger" role="alert">
        <strong>Error!</strong> Failed to load games:
        {{ response.error.message || 'Unknown error' }}
    </div>
    <div v-else class="container mt-4">
        <h2 class="mb-4 text-center">Games Overview</h2>
        <div v-if="response.payload.games.length === 0" class="text-center text-muted">
            No games available.
        </div>
        <div v-else class="row">
            <div
                v-for="game in response.payload.games"
                :key="game.id"
                class="col-lg-4 col-md-6 mb-4"
            >
                <div class="card h-100 shadow-sm border-0 bg-light">
                    <div class="card-body d-flex flex-column">
                        <h5 class="card-title text-primary fw-bold">{{ game.name }}</h5>
                        <div class="mb-2">
                            <span class="badge bg-primary me-2"
                                >Players: {{ game.number_of_players }}</span
                            >
                            <span class="badge bg-secondary me-2"
                                >Round: {{ game.round_number }}</span
                            >
                        </div>
                        <div class="mt-auto">
                            <button class="btn btn-outline-primary btn-sm w-100">Resume</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
