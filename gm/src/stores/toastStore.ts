import { defineStore } from 'pinia';

// incremental id for new modals
let id = 0;

export const useToastStore = defineStore('toast', {
    state: () => ({
        toasts: new Map<
            number,
            {
                message: string;
                type: 'success' | 'error' | 'info' | 'warning';
                duration: number | null;
            }
        >(),
    }),
    actions: {
        addToast(
            message: string,
            type: 'success' | 'error' | 'info' | 'warning' = 'info',
            duration: number | null,
        ): void {
            const toastId = id++;

            this.toasts.set(toastId, { message, type, duration: duration });

            // And set up a timeout to remove it after 'duration' milliseconds
            if (duration !== null) {
                setTimeout(() => {
                    this.toasts.delete(toastId);
                }, duration);
            }
        },
        removeToast(toastId: number): void {
            this.toasts.delete(toastId);
        },
    },
});
