// Mock document.cookie for Node.js environment
globalThis.document = Object.defineProperty(globalThis.document || {}, 'cookie', {
    writable: true,
    value: '',
});

import { vi, describe, it, expect, afterEach } from 'vitest';
import { request, api } from '../../src/api/httpApi';
import AxiosMockAdapter from 'axios-mock-adapter';
import { nextTick } from 'vue';

describe('httpApi request', () => {
    const mock = new AxiosMockAdapter(api);

    afterEach(() => {
        vi.clearAllMocks();
        mock.reset();
    });

    it('should return a Ref with loading true initially', () => {
        const resultRef = request('GET', '/test-endpoint');

        expect(resultRef.value.loading).toBe(true);
        expect(resultRef.value.data).toBeNull();
        expect(resultRef.value.error).toBeNull();
    });

    it('should handle successful request', async () => {
        const mockData = { message: 'Success', data: { id: 1, name: 'Test' } };
        mock.onGet('/test-endpoint').reply(200, mockData);

        const resultRef = request('GET', '/test-endpoint');

        // Wait for axios promise to resolve
        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.error).toBeNull();
        expect(resultRef.value.data).toEqual(mockData);
    });

    it('should handle failed request with axios error', async () => {
        mock.onPost('/test-endpoint').networkError();

        const resultRef = request('POST', '/test-endpoint');

        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.data).toBeNull();
        expect(resultRef.value.error).toBeInstanceOf(Error);
        expect(resultRef.value.error!.message).toBe('Network Error');
    });

    it('should handle failed request with server error', async () => {
        const mockErrorResponse = {
            response: {
                data: { error: 'Test Server Error' },
                statusText: 'Bad Request',
            },
        };
        mock.onGet('/error-endpoint').reply(400, mockErrorResponse.response.data);

        const resultRef = request('GET', '/error-endpoint');

        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.data).toBeNull();
        expect(resultRef.value.error).toBeInstanceOf(Error);
        expect(resultRef.value.error!.message).toBe('Test Server Error');
    });

    it('should add Authorization header if token exists', async () => {
        // Set a token in the mock cookie
        document.cookie = 'token=test-token';

        const mockData = { message: 'Authorized' };
        mock.onGet('/auth-endpoint').reply(config => {
            expect(config.headers?.Authorization).toBe('Bearer test-token');
            return [200, mockData];
        });

        const resultRef = request('GET', '/auth-endpoint');

        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.error).toBeNull();
        expect(resultRef.value.data).toEqual(mockData);
    });

    it('should not add Authorization header if no token exists', async () => {
        // Clear the mock cookie
        document.cookie = '';

        const mockData = { message: 'No Auth' };
        mock.onGet('/no-auth-endpoint').reply(config => {
            expect(config.headers?.Authorization).toBeUndefined();
            return [200, mockData];
        });

        const resultRef = request('GET', '/no-auth-endpoint');

        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.error).toBeNull();
        expect(resultRef.value.data).toEqual(mockData);
    });

    it('should add a request body when provided', async () => {
        const requestBody = { name: 'Test' };
        const mockResponse = { message: 'Created', id: 1 };

        mock.onPost('/create-endpoint', requestBody).reply(config => {
            expect(JSON.parse(config.data)).toEqual(requestBody);
            return [201, mockResponse];
        });

        const resultRef = request('POST', '/create-endpoint', requestBody);

        await new Promise(resolve => setTimeout(resolve, 0));
        await nextTick();

        expect(resultRef.value.loading).toBe(false);
        expect(resultRef.value.error).toBeNull();
        expect(resultRef.value.data).toEqual(mockResponse);
    });
});
