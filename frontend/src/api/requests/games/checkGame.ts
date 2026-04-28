import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import { getWithAuth } from '../../httpApi';

export type CheckGamesResponse =
    | {
          is_valid: true;
          output: unknown; // For now, we only output the stringified output.
      }
    | {
          is_valid: false;
          errors: string[];
      };

const isValidResponse = (res: unknown): res is CheckGamesResponse => {
    if (typeof res !== 'object' || res === null) {
        return false;
    }

    const obj = res as Record<string, unknown>;

    if (typeof obj.is_valid !== 'boolean') {
        return false;
    }

    if (obj.is_valid === true) {
        if (!('output' in obj)) {
            return false;
        }

        return true;
    } else {
        if (!Array.isArray(obj.errors)) {
            return false;
        }
        if (!obj.errors.every(error => typeof error === 'string')) {
            return false;
        }
        return true;
    }
};

export const checkGame = (gameId: string): ResultAsync<CheckGamesResponse, string> => {
    return getWithAuth<CheckGamesResponse>('/games/' + gameId + '/check').andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        return okAsync(res.data);
    });
};
