<template>
    <div class="container mt-5" style="max-width: 400px">
        <div class="card shadow-sm">
            <div class="card-body">
                <h2 class="card-title text-center mb-4" v-if="mode === 'login'">Login</h2>
                <h2 class="card-title text-center mb-4" v-else>Register</h2>
                <form @submit.prevent="handleSubmit">
                    <div class="mb-3">
                        <label for="name" class="form-label">Username</label>
                        <input v-model="form.name" id="name" class="form-control" required />
                    </div>
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
                    <button type="submit" class="btn btn-primary w-100 mb-2">
                        {{ mode === 'login' ? 'Login' : 'Register' }}
                    </button>
                </form>
                <button class="btn btn-link w-100 p-0" @click="toggleMode">
                    {{
                        mode === 'login'
                            ? 'Need an account? Register'
                            : 'Already have an account? Login'
                    }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useAuth } from '../composables/useAuth';
const { mode, form, error, toggleMode, handleSubmit } = useAuth();
</script>
