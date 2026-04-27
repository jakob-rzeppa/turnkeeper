import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { postWithAuth } from '../../httpApi';

interface CreateGameResponse {
    id: string;
}

const isValidResponse = (res: unknown): res is CreateGameResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const g = res as Record<string, unknown>;
    return typeof g.id === 'string';
};

export const createGame = (name: string, description: string): ResultAsync<string, string> => {
    return postWithAuth<CreateGameResponse>(`/games`, {
        name,
        description,
    }).andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        return okAsync(res.data.id);
    });
};
