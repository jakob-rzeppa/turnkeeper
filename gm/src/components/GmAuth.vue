<script setup lang="ts">
import { useAuth } from '../composables/useAuth';
const { form, loading, error, handleSubmit } = useAuth();
</script>

<template>
    <div class="container mt-5" style="max-width: 400px">
        <div class="card shadow-sm">
            <div class="card-body">
                <h2 class="card-title text-center mb-4">Login</h2>

                <div v-if="loading" class="d-flex justify-content-center my-4">
                    <div class="spinner-border" role="status">
                        <span class="visually-hidden">Loading...</span>
                    </div>
                </div>

                <div v-else>
                    <form @submit.prevent="handleSubmit">
                        <div class="mb-3">
                            <label for="password" class="form-label">Password</label>
                            <input
                                v-model="form.password"
                                id="password"
                                type="password"
                                class="form-control"
                                required
                            />
                        </div>
                        <div v-if="error" class="alert alert-danger py-2">{{ error }}</div>
                        <button type="submit" class="btn btn-primary w-100 mb-2">Login</button>
                    </form>
                </div>
            </div>
        </div>
    </div>
</template>
