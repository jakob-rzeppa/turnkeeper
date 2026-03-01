import { defineStore } from 'pinia';
import { computed, ref, type ComputedRef } from 'vue';
import { useAuthStore } from '../auth/authStore';
import { API_BASE_URL } from '../api/httpApi';

export type User = {
    id: string;
    name: string;
};

export const useUsersStore = defineStore('users', () => {
    const authStore = useAuthStore();

    const users = ref<User[]>([]);

    async function loadUsers() {
        // real call to backend service
        try {
            const resp = await fetch(API_BASE_URL + '/users', {
                method: 'GET',
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            });
            if (!resp.ok) {
                throw new Error(`fetch failed: ${resp.status} ${resp.statusText}`);
            }
            const data: { users: User[] } = await resp.json();
            users.value = data.users;
        } catch (err) {
            console.error('could not load users', err);
        }
    }

    const getById = (id: string): ComputedRef<User | undefined> => {
        return computed(() => users.value.find(user => user.id === id));
    };

    return { users, loadUsers, getById };
});
