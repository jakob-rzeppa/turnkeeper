import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import type { GameMetadata } from '../../../types/game';
import { getWithAuth } from '../../httpApi';

interface GetGamesResponse {
    games: {
        id: string;
        name: string;
        description: string;

        created_at: string;
        updated_at: string;
    }[];
}

const isValidResponse = (res: unknown): res is GetGamesResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const g = res as { games: Record<string, unknown>[] };
    return (
        Array.isArray(g.games) &&
        g.games.every(
            game =>
                typeof game.id === 'string' &&
                typeof game.name === 'string' &&
                typeof game.description === 'string' &&
                typeof game.created_at === 'string' &&
                typeof game.updated_at === 'string'
        )
    );
};

export const getGames = (): ResultAsync<GameMetadata[], string> => {
    return getWithAuth<GetGamesResponse>('/games').andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        const games: GameMetadata[] = res.data.games;

        return okAsync(games);
    });
};
