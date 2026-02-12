<script setup lang="ts">
import { useAuth } from './useAuth';
const { form, loading, error, handleSubmit } = useAuth();
</script>

<template>
    <div class="max-w-md mx-auto">
        <div class="card bg-base-100 shadow-md">
            <div class="card-body">
                <h2 class="text-xl font-bold text-center">Login</h2>

                <template v-if="loading">
                    <span class="mx-auto loading loading-spinner loading-lg"></span>
                </template>

                <template v-else>
                    <form @submit.prevent="handleSubmit" novalidate class="space-y-4">
                        <label class="input w-full">
                            <span class="label">Password</span>
                            <input
                                v-model="form.password"
                                type="password"
                                id="password"
                                placeholder="Enter password"
                            />
                        </label>

                        <div v-if="error" class="alert alert-error py-2">{{ error }}</div>

                        <button type="submit" class="btn btn-primary w-full mb-2">Login</button>
                    </form>
                </template>
            </div>
        </div>
    </div>
</template>
