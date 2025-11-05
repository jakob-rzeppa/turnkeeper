import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToGmEventPayloads, Message } from 'shared-types';
import { ref } from 'vue';

const connection = useConnection();

export const useMessagesStore = defineStore('messages', () => {
    const messages = ref<{ [key: number]: Message[] }>({});

    connection.socket.on(
        'messages:all',
        ({ messages: newMessages }: BackendToGmEventPayloads['messages:all']) => {
            messages.value = newMessages;
        },
    );

    return { messages };
});
