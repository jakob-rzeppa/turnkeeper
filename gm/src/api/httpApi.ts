// Centralized HTTP API handler for Turnkeeper
import { type Ref } from '@vue/runtime-dom';
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios';

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

export const api = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

// Get JWT token from localStorage (or other storage if needed)
function getToken(): string | null {
    return (
        document.cookie
            .split('; ')
            .find(row => row.startsWith('token='))
            ?.split('=')[1] || null
    );
}

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
export type ResponseStatus<T> =
    | { loading: true; payload: null; error: null }
    | { loading: false; payload: T; error: null }
    | { loading: false; payload: null; error: Error };

export function getDefaultResponseStatus<T>(): ResponseStatus<T> {
    return { loading: true, payload: null, error: null };
}

// Generic request handler with Result type
export function request<T = object>(
    statusRef: Ref<ResponseStatus<T>>,
    method: AxiosRequestConfig['method'],
    url: string,
    data?: object,
    config?: AxiosRequestConfig
): void {
    statusRef.value = { loading: true, payload: null, error: null };

    api({
        method,
        url,
        data,
        ...config,
    })
        .then((res: AxiosResponse<T>) => {
            statusRef.value = { loading: false, payload: res.data, error: null };
        })
        .catch((err: Error) => {
            statusRef.value = { loading: false, payload: null, error: err };
        });
}
