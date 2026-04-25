import { defineStore } from 'pinia';
import { ref } from 'vue';
import { deleteWithAuth, getWithAuth, postWithAuth } from '../api/httpApi';
import type { GameMetadata } from '../types/game';

export const useGamesStore = defineStore('games', () => {
    const data = ref<
        | { loading: true }
        | { loading: false; error: string }
        | { loading: false; error: null; games: GameMetadata[] }
    >({ loading: true });

    const loadGames = async () => {
        data.value = { loading: true };
        const res = await getWithAuth<{ games: GameMetadata[] }>('/games').map(
            result => result.games
        );

        if (res.isOk()) {
            data.value = { loading: false, error: null, games: res.value };
        } else {
            data.value = { loading: false, error: res.error.message };
            return;
        }
    };

    const createGame = async (name: string, description: string) => {
        data.value = { loading: true };
        const res = await postWithAuth<GameMetadata>('/games', { name, description }).map(
            game => game.id
        );

        if (res.isOk()) {
            loadGames();
        } else {
            data.value = { loading: false, error: res.error.message };
        }
    };

    const deleteGame = async (gameId: string) => {
        data.value = { loading: true };
        const res = await deleteWithAuth(`/games/${gameId}`);

        if (res.isOk()) {
            loadGames();
        } else {
            data.value = { loading: false, error: res.error.message };
        }
    };

    loadGames();

    return {
        data,
        loadGames,
        createGame,
        deleteGame,
    };
});
