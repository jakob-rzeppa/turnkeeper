import { ref } from 'vue';
import type { GameMetadata } from '../types/game';
import { API_BASE_URL, apiErrorToMessage } from '../api/httpApi';
import axios from 'axios';
import { useAuthStore } from '../auth/authStore';

export const useGameOverview = () => {
    const authStore = useAuthStore();

    const games = ref<GameMetadata[]>([]);
    const loading = ref(false);
    const error = ref('');

    const fetchGames = async () => {
        loading.value = true;
        error.value = '';

        try {
            const response = await axios.get<{ games: GameMetadata[] }>(API_BASE_URL + '/games', {
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            });
            games.value = response.data.games;
        } catch (err: unknown) {
            error.value = 'Failed to fetch games: ' + apiErrorToMessage(err);
        } finally {
            loading.value = false;
        }
    };

    // Fetch games on composable initialization
    fetchGames();

    return {
        games,
        loading,
        error,
        fetchGames,
    };
};
