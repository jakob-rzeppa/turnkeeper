// Centralized HTTP API handler for Turnkeeper
import { ref, type Ref } from '@vue/runtime-dom';
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

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
export type RequestStatus<T, E = Error> =
    | { loading: true; payload: null; error: null }
    | { loading: false; payload: T; error: null }
    | { loading: false; payload: null; error: E };

// Generic request handler with Result type
export function request<T = object>(
    method: AxiosRequestConfig['method'],
    url: string,
    data?: object,
    config?: AxiosRequestConfig
): Ref<RequestStatus<T>> {
    const response: Ref<RequestStatus<T>> = ref({ loading: true, payload: null, error: null });

    api({
        method,
        url,
        data,
        ...config,
    })
        .then((res: AxiosResponse<T>) => {
            response.value = { loading: false, payload: res.data, error: null };
        })
        .catch((err: Error) => {
            response.value = { loading: false, payload: null, error: err };
        });

    return response;
}
