import { AxiosError } from 'axios';

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/gm';

export function apiErrorToMessage(error: unknown): string {
    if (error instanceof AxiosError) {
        return error.response?.data?.error ?? error.message;
    }

    if (error instanceof Error) {
        return error.message;
    }

    return 'Unknown error';
}
