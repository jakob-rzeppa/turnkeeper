import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { postWithAuth } from '../../httpApi';

interface CreateGameInstanceResponse {
    id: string;
}

const isValidResponse = (res: unknown): res is CreateGameInstanceResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const g = res as Record<string, unknown>;
    return typeof g.id === 'string';
};

export const createGameInstance = (gameId: string, name: string): ResultAsync<string, string> => {
    return postWithAuth<CreateGameInstanceResponse>(`/games/${gameId}/instances`, {
        name,
    }).andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        return okAsync(res.data.id);
    });
};
