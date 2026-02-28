import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

function getTokenFromCookie(): string | null {
    const match = document.cookie.match(/(?:^|; )user_token=([^;]*)/) as string[] | null;
    return match ? decodeURIComponent(match[1]!) : null;
}

export const useAuthStore = defineStore('auth', () => {
    const token = ref<string | null>(getTokenFromCookie());

    const isAuthenticated = computed(() => token.value !== null);

    const setToken = (newToken: string) => {
        token.value = newToken;
        document.cookie = `user_token=${encodeURIComponent(newToken)}; path=/`;
        console.log('Auth token set in store and cookie');
    };

    const clearToken = () => {
        token.value = null;
        document.cookie = 'user_token=; path=/;';
        console.log('Auth token cleared from store and cookie');
    };

    const syncWithCookie = () => {
        const cookieToken = getTokenFromCookie();
        if (cookieToken !== token.value) {
            token.value = cookieToken;
        }
        console.log('Auth token synchronized with cookie');
    };

    return {
        token,
        isAuthenticated,
        setToken,
        clearToken,
        syncWithCookie,
    };
});
