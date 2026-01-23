import { ref, watch } from 'vue';
import { request } from './useHttpApi';
import { useAuthStore } from '../stores/auth';

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

    function handleSubmit() {
        error.value = '';
        const endpoint = mode.value === 'login' ? '/user/login' : '/user/register';
        const payload = { ...form.value };
        const resultRef = request<{ token: string }>('POST', endpoint, payload);
        loading.value = true;

        // Watch for changes in the resultRef
        const stop = watch(
            resultRef,
            result => {
                if (!result.loading) {
                    if (result.value) {
                        authStore.setToken(result.value.token);
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
        mode,
        form,
        loading,
        error,
        toggleMode,
        handleSubmit,
    };
}
