import { ref } from 'vue';
import { request } from '../services/httpApi';
import { useAuthStore } from '../stores/auth';

export function useAuth() {
    const authStore = useAuthStore();

    const mode = ref<'login' | 'register'>('login');
    const form = ref({ name: '', password: '' });
    const error = ref('');

    // Sync store token with cookie on composable initialization.
    // This way if the user enters the login page and for some reason the cookie is set,
    // the store will reflect that state and automatically consider the user as logged in.
    authStore.syncWithCookie();

    function toggleMode() {
        mode.value = mode.value === 'login' ? 'register' : 'login';
        error.value = '';
    }

    async function handleSubmit() {
        error.value = '';
        const endpoint = mode.value === 'login' ? '/user/login' : '/user/register';
        const payload = { ...form.value };

        const result = await request<{ token: string; message?: string }>(
            'POST',
            endpoint,
            payload
        );

        if (result.ok) {
            authStore.setToken(result.value.token);
        } else {
            error.value = result.error.message || 'Unknown error';
        }
    }

    return {
        mode,
        form,
        error,
        toggleMode,
        handleSubmit,
    };
}
