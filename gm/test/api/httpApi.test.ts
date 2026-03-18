import { vi, describe, it, expect, afterEach } from 'vitest';
import AxiosMockAdapter from 'axios-mock-adapter';
import { apiErrorToMessage } from '../../src/api/httpApi';
import axios from 'axios';

describe('httpApi', () => {
    const mock = new AxiosMockAdapter(axios);

    afterEach(() => {
        vi.clearAllMocks();
        mock.reset();
    });

    describe('apiErrorToMessage', () => {
        it('should return error message from AxiosError response if response object with error is present', async () => {
            const errorMessage = 'Request failed with status code 500';
            mock.onPost('/test-endpoint').reply(500, { message: errorMessage });

            try {
                await axios.post('/test-endpoint');
            } catch (error) {
                const message = apiErrorToMessage(error);
                expect(message).toBe(errorMessage);
            }
        });

        it('should return error message from AxiosError response if no response object with error is present', async () => {
            mock.onPost('/test-endpoint').networkError();

            try {
                await axios.post('/test-endpoint');
            } catch (error) {
                const message = apiErrorToMessage(error);
                expect(message).toBe('Network Error');
            }
        });
    });
});
