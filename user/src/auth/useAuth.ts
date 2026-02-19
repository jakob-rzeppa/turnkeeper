import { ref } from 'vue';
import { useAuthStore } from './authStore';
import axios from 'axios';
import { API_BASE_URL, apiErrorToMessage } from '../api/httpApi';

export function useAuth() {
    const authStore = useAuthStore();

    const mode = ref<'login' | 'register'>('login');
    const form = ref({ name: '', password: '' });
    const loading = ref(false);
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
        loading.value = true;

        const payload = { name: form.value.name, password: form.value.password };

        try {
            const response = await axios.post<{ token: string }>(
                API_BASE_URL + '/user/login',
                payload
            );

            authStore.setToken(response.data.token);
        } catch (err: unknown) {
            error.value = 'Login failed: ' + apiErrorToMessage(err);
        } finally {
            loading.value = false;
        }
    }

    return {
        mode,
        form,
        loading,
        error,
        toggleMode,
        handleSubmit,
    };
}
