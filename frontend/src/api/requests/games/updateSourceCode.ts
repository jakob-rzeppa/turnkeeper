import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { patchWithAuth } from '../../httpApi';

export const updateSourceCode = (gameId: string, sourceCode: string): ResultAsync<void, string> => {
    return patchWithAuth(`/games/${gameId}/source-code`, {
        source_code: sourceCode,
    }).andThen(res => {
        if (res.status === 204) {
            return okAsync();
        } else {
            return errAsync('Response status was not 204.');
        }
    });
};
