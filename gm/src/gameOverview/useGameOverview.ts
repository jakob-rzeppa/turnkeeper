import { ref } from 'vue';
import type { GameMetadata } from '../types/game';
import { API_BASE_URL } from '../api/httpApi';
import axios from 'axios';

export const useGameOverview = () => {
    const games = ref<GameMetadata[]>([]);
    const loading = ref(false);
    const error = ref('');

    const fetchGames = async () => {
        loading.value = true;
        error.value = '';

        try {
            const response = await axios.get<{ games: GameMetadata[] }>(API_BASE_URL + '/games');
            games.value = response.data.games;
        } catch (err: unknown) {
            error.value = err instanceof Error ? err.message : 'Unknown error';
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
