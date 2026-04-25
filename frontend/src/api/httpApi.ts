import axios, { AxiosError } from 'axios';
import { ResultAsync } from 'neverthrow';
import { useAuthStore } from '../auth/authStore';
import type { HttpError } from '../errors/httpError';

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

export const getWithAuth = <T>(endpoint: string): ResultAsync<T, HttpError> => {
    const authStore = useAuthStore();

    return ResultAsync.fromPromise(
        axios
            .get<T>(API_BASE_URL + endpoint, {
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            })
            .then(res => res.data),
        err => {
            if (err instanceof AxiosError) {
                return { message: err.response?.data?.error ?? err.message };
            }

            if (err instanceof Error) {
                return { message: err.message };
            }

            return { message: 'Unknown error' };
        }
    );
};

export const postWithAuth = <T>(endpoint: string, data: unknown): ResultAsync<T, HttpError> => {
    const authStore = useAuthStore();

    return ResultAsync.fromPromise(
        axios
            .post<T>(API_BASE_URL + endpoint, data, {
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            })
            .then(res => res.data),
        err => {
            if (err instanceof AxiosError) {
                return { message: err.response?.data?.error ?? err.message };
            }

            if (err instanceof Error) {
                return { message: err.message };
            }

            return { message: 'Unknown error' };
        }
    );
};

export const deleteWithAuth = <T>(endpoint: string): ResultAsync<T, HttpError> => {
    const authStore = useAuthStore();

    return ResultAsync.fromPromise(
        axios
            .delete<T>(API_BASE_URL + endpoint, {
                headers: {
                    Authorization: 'Bearer ' + authStore.token,
                },
            })
            .then(res => res.data),
        err => {
            if (err instanceof AxiosError) {
                return { message: err.response?.data?.error ?? err.message };
            }

            if (err instanceof Error) {
                return { message: err.message };
            }

            return { message: 'Unknown error' };
        }
    );
};

export function apiErrorToMessage(error: unknown): string {
    if (error instanceof AxiosError) {
        return error.response?.data?.error ?? error.message;
    }

    if (error instanceof Error) {
        return error.message;
    }

    return 'Unknown error';
}
