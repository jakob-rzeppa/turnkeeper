// Mock document.cookie for Node.js environment
globalThis.document = Object.defineProperty(globalThis.document || {}, 'cookie', {
    writable: true,
    value: '',
});

import { vi, describe, it, expect, afterEach } from 'vitest';
import { request, api } from '../../src/services/httpApi';
import AxiosMockAdapter from 'axios-mock-adapter';

describe('httpApi request', () => {
    const mock = new AxiosMockAdapter(api);

    afterEach(() => {
        vi.clearAllMocks();
        mock.reset();
    });

    it('should handle successful request', async () => {
        const mockData = { message: 'Success', data: { id: 1, name: 'Test' } };
        mock.onGet('/test-endpoint').reply(200, mockData);

        const result = await request('GET', '/test-endpoint');

        console.log(result);
        expect(result.ok).toBe(true);
        if (result.ok) {
            expect(result.value).toEqual(mockData);
        }
    });

    it('should handle failed request with axios error', async () => {
        mock.onPost('/test-endpoint').networkError();

        const result = await request('POST', '/test-endpoint');

        expect(result.ok).toBe(false);
        if (!result.ok) {
            expect(result.error).toBeInstanceOf(Error);
            expect(result.error.message).toBe('Network Error');
        }
    });

    it('should handle failed request with server error', async () => {
        const mockErrorResponse = {
            response: {
                data: { error: 'Test Server Error' },
                statusText: 'Bad Request',
            },
        };
        mock.onGet('/error-endpoint').reply(400, mockErrorResponse.response.data);

        const result = await request('GET', '/error-endpoint');

        expect(result.ok).toBe(false);
        if (!result.ok) {
            expect(result.error).toBeInstanceOf(Error);
            expect(result.error.message).toBe('Test Server Error');
        }
    });

    it('should add Authorization header if token exists', async () => {
        // Set a token in the mock cookie
        document.cookie = 'token=test-token';

        const mockData = { message: 'Authorized' };
        mock.onGet('/auth-endpoint').reply(config => {
            expect(config.headers?.Authorization).toBe('Bearer test-token');
            return [200, mockData];
        });

        const result = await request('GET', '/auth-endpoint');

        expect(result.ok).toBe(true);
        if (result.ok) {
            expect(result.value).toEqual(mockData);
        }
    });

    it('should not add Authorization header if no token exists', async () => {
        // Clear the mock cookie
        document.cookie = '';

        const mockData = { message: 'No Auth' };
        mock.onGet('/no-auth-endpoint').reply(config => {
            expect(config.headers?.Authorization).toBeUndefined();
            return [200, mockData];
        });

        const result = await request('GET', '/no-auth-endpoint');

        expect(result.ok).toBe(true);
        if (result.ok) {
            expect(result.value).toEqual(mockData);
        }
    });

    it('should add a request body when provided', async () => {
        const requestBody = { name: 'Test' };
        const mockResponse = { message: 'Created', id: 1 };

        mock.onPost('/create-endpoint', requestBody).reply(config => {
            expect(JSON.parse(config.data)).toEqual(requestBody);
            return [201, mockResponse];
        });

        const result = await request('POST', '/create-endpoint', requestBody);

        expect(result.ok).toBe(true);
        if (result.ok) {
            expect(result.value).toEqual(mockResponse);
        }
    });
});
