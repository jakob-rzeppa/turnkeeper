import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { deleteWithAuth } from '../../httpApi';

export const deleteGameInstance = (
    gameId: string,
    gameInstanceId: string
): ResultAsync<void, string> => {
    return deleteWithAuth(`/games/${gameId}/instances/${gameInstanceId}`).andThen(res => {
        if (res.status === 204) {
            return okAsync();
        } else {
            return errAsync('Response status was not 204.');
        }
    });
};
