import { ref } from 'vue';
import { useAuthStore } from './authStore';
import axios from 'axios';
import { API_BASE_URL, apiErrorToMessage } from '../api/httpApi';

export function useAuth() {
    const authStore = useAuthStore();

    const form = ref({ password: '' });
    const loading = ref(false);
    const error = ref('');

    // Sync store token with cookie on composable initialization.
    // This way if the gm enters the login page and for some reason the cookie is set,
    // the store will reflect that state and automatically consider the gm as logged in.
    authStore.syncWithCookie();

    async function handleSubmit() {
        error.value = '';
        loading.value = true;

        const payload = { password: form.value.password };

        try {
            const response = await axios.post<{ token: string }>(
                API_BASE_URL + '/gm/login',
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
        form,
        loading,
        error,
        handleSubmit,
    };
}
