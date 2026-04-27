import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import type { GameDetails } from '../../../types/game';
import { getWithAuth } from '../../httpApi';

interface GetGameDetailsResponse {
    id: string;
    name: string;
    description: string;
    source_code: string;
    created_at: string;
    updated_at: string;
}

const isValidResponse = (res: unknown): res is GetGameDetailsResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const g = res as Record<string, unknown>;
    return (
        typeof g.id === 'string' &&
        typeof g.name === 'string' &&
        typeof g.description === 'string' &&
        typeof g.source_code === 'string' &&
        typeof g.created_at === 'string' &&
        typeof g.updated_at === 'string'
    );
};

export const getGameDetails = (gameId: string): ResultAsync<GameDetails, string> => {
    return getWithAuth<GetGameDetailsResponse>(`/games/${gameId}`).andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        const game: GameDetails = res.data;

        return okAsync(game);
    });
};
