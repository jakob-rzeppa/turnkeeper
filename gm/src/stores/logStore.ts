import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads, LogEntry } from 'shared-types';
import { defineStore } from 'pinia';
import { ref } from 'vue';

const { socket } = useConnection();

export const useLogStore = defineStore('log', () => {
    // The store shall only be modified by events from the backend.
    const logs = ref([] as LogEntry[]);

    socket.on('log:entry', ({ entry }: BackendToGmEventPayloads['log:entry']) => {
        logs.value.push(entry);
    });

    return {
        logs,
    };
});
