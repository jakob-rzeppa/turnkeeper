// Centralized HTTP API handler for Turnkeeper
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

// Get JWT token from localStorage (or other storage if needed)
function getToken(): string | null {
    return (
        document.cookie
            .split('; ')
            .find(row => row.startsWith('token='))
            ?.split('=')[1] || null
    );
}

export const api = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

// Request interceptor to add Authorization header if token exists
api.interceptors.request.use(config => {
    const token = getToken();
    if (token) {
        config.headers = config.headers || {};
        config.headers['Authorization'] = `Bearer ${token}`;
    }
    return config;
});

// Response interceptor for error handling
api.interceptors.response.use(
    response => response,
    error => {
        if (error.response) {
            // API returned an error response
            const apiError = error.response.data?.error || error.response.statusText;
            return Promise.reject(new Error(apiError));
        } else if (error.request) {
            // No response received
            return Promise.reject(new Error('No response from server'));
        } else {
            // Other errors
            return Promise.reject(new Error(error.message));
        }
    }
);

// Result type for error-as-value handling
export type HttpResult<T, E = Error> = { ok: true; value: T } | { ok: false; error: E };

// Generic request handler with Result type
export async function request<T = object>(
    method: AxiosRequestConfig['method'],
    url: string,
    data?: object,
    config?: AxiosRequestConfig
): Promise<HttpResult<T>> {
    try {
        const response: AxiosResponse<T> = await api({
            method,
            url,
            data,
            ...config,
        });
        return { ok: true, value: response.data };
    } catch (err: unknown) {
        if (err instanceof Error) return { ok: false, error: err };

        return { ok: false, error: new Error('Unknown error occurred') };
    }
}
