import { defineStore } from 'pinia';

function getTokenFromCookie(): string | null {
    const match = document.cookie.match(/(?:^|; )token=([^;]*)/) as string[] | null;
    return match ? decodeURIComponent(match[1]!) : null;
}

export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: getTokenFromCookie() as string | null,
    }),
    getters: {
        isAuthenticated: state => state.token !== null,
    },
    actions: {
        setToken(newToken: string) {
            this.token = newToken;
            document.cookie = `token=${encodeURIComponent(newToken)}; path=/`;
            console.log('Auth token set in store and cookie');
        },
        clearToken() {
            this.token = null;
            document.cookie = 'token=; path=/;';
            console.log('Auth token cleared from store and cookie');
        },
        syncWithCookie() {
            const cookieToken = getTokenFromCookie();
            if (cookieToken !== this.token) {
                this.token = cookieToken;
            }
            console.log('Auth token synchronized with cookie');
        },
    },
});
