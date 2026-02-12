import { ref, watch } from 'vue';
import { request } from '../api/httpApi';
import { useAuthStore } from './authStore';

export function useAuth() {
    const authStore = useAuthStore();

    const form = ref({ password: '' });
    const loading = ref(false);
    const error = ref('');

    // Sync store token with cookie on composable initialization.
    // This way if the gm enters the login page and for some reason the cookie is set,
    // the store will reflect that state and automatically consider the gm as logged in.
    authStore.syncWithCookie();

    function handleSubmit() {
        error.value = '';
        const payload = { ...form.value };
        const resultRef = request<{ token: string }>('POST', '/gm/login', payload);
        loading.value = true;

        // Watch for changes in the resultRef
        const stop = watch(
            resultRef,
            result => {
                if (!result.loading) {
                    if (result.payload) {
                        authStore.setToken(result.payload.token);
                        stop();
                    } else if (result.error) {
                        error.value = result.error.message || 'Unknown error';
                        stop();
                    }
                }

                loading.value = result.loading;
            },
            { immediate: true, deep: true }
        );
    }

    return {
        form,
        loading,
        error,
        handleSubmit,
    };
}
