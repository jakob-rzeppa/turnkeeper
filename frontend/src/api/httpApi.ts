import axios, { AxiosError, type AxiosResponse } from 'axios';
import { ResultAsync } from 'neverthrow';
import { useAuthStore } from '../auth/authStore';

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

interface ErrorResponse {
    error: string;
}

const getAuthHeader = () => {
    const authStore = useAuthStore();
    return {
        headers: {
            Authorization: 'Bearer ' + authStore.token,
        },
    };
};

const isErrorResponse = (res: unknown): res is ErrorResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const r = res as Record<string, unknown>;
    return 'error' in r && typeof r.error === 'string';
};

const handleApiError = (err: unknown): string => {
    if (err instanceof AxiosError) {
        if (isErrorResponse(err.response?.data)) {
            return err.response.data.error;
        }

        return err.message;
    }

    if (err instanceof Error) {
        return err.message;
    }

    return 'Unknown error';
};

export const getWithAuth = <T>(endpoint: string): ResultAsync<AxiosResponse<T>, string> => {
    return ResultAsync.fromPromise(axios.get<T>(API_BASE_URL + endpoint, getAuthHeader()), err =>
        handleApiError(err)
    );
};

export const postWithAuth = <T>(
    endpoint: string,
    data: unknown
): ResultAsync<AxiosResponse<T>, string> => {
    return ResultAsync.fromPromise(
        axios.post<T>(API_BASE_URL + endpoint, data, getAuthHeader()),
        err => handleApiError(err)
    );
};

export const patchWithAuth = <T>(
    endpoint: string,
    data: unknown
): ResultAsync<AxiosResponse<T>, string> => {
    return ResultAsync.fromPromise(
        axios.patch<T>(API_BASE_URL + endpoint, data, getAuthHeader()),
        err => handleApiError(err)
    );
};

export const deleteWithAuth = <T>(endpoint: string): ResultAsync<AxiosResponse<T>, string> => {
    return ResultAsync.fromPromise(axios.delete<T>(API_BASE_URL + endpoint, getAuthHeader()), err =>
        handleApiError(err)
    );
};
