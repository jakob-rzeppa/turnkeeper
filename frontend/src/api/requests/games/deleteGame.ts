import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { deleteWithAuth } from '../../httpApi';

export const deleteGame = (gameId: string): ResultAsync<void, string> => {
    return deleteWithAuth(`/games/${gameId}`).andThen(res => {
        if (res.status === 204) {
            return okAsync();
        } else {
            return errAsync('Response status was not 204.');
        }
    });
};
