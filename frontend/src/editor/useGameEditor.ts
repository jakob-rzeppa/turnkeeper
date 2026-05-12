import { computed, readonly, ref } from 'vue';
import type { DataState } from '../types/util';
import type { GameDetails } from '../types/game';
import { checkGame, type CheckGamesResponse } from '../api/requests/games/checkGame';
import { getGameDetails } from '../api/requests/games/getGameDetails';
import { updateSourceCode } from '../api/requests/games/updateSourceCode';

const game = ref<DataState<GameDetails>>({ status: 'uninitialized' });
const checkResult = ref<DataState<CheckGamesResponse>>({ status: 'uninitialized' });

const sourceCode = ref<string>('');

export const useGameEditor = () => {
    const isSaved = computed(() => {
        if (game.value.status !== 'success') return false;
        return sourceCode.value === game.value.data.source_code;
    });

    const loadGame = async (gameId: string) => {
        game.value = { status: 'loading' };
        const res = await getGameDetails(gameId);

        if (res.isOk()) {
            game.value = { status: 'success', data: res.value };
            checkResult.value = { status: 'uninitialized' };
            sourceCode.value = res.value.source_code;
        } else {
            game.value = { status: 'error', error: res.error };
        }
    };

    const saveSourceCode = async (gameId: string) => {
        if (
            game.value.status === 'uninitialized' ||
            game.value.status === 'loading' ||
            game.value.status === 'error'
        )
            return;

        if (gameId !== game.value.data.id) {
            game.value = { status: 'error', error: 'Game ID mismatch' };
            return;
        }

        game.value = { status: 'loading' };

        const res = await updateSourceCode(gameId, sourceCode.value);

        if (res.isOk()) {
            loadGame(gameId);
        } else {
            game.value = { status: 'error', error: res.error };
        }
    };

    const checkSourceCode = async (gameId: string) => {
        if (
            game.value.status === 'uninitialized' ||
            game.value.status === 'loading' ||
            game.value.status === 'error'
        )
            return;

        if (gameId !== game.value.data.id) {
            game.value = { status: 'error', error: 'Game ID mismatch' };
            return;
        }

        checkResult.value = { status: 'loading' };
        const res = await checkGame(gameId);

        if (res.isOk()) {
            checkResult.value = { status: 'success', data: res.value };
        } else {
            checkResult.value = { status: 'error', error: res.error };
        }
    };

    return {
        game: readonly(game),
        sourceCode,
        isSaved,
        checkResult: readonly(checkResult),
        loadGame,
        saveSourceCode,
        checkSourceCode,
    };
};
