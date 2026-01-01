import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToUserEventPayloads, Message } from 'shared-types';
import { ref } from 'vue';

const connection = useConnection();

export const useMessagesStore = defineStore('messages', () => {
    const messages = ref<Message[]>([]);

    connection.socket.on(
        'messages:all',
        ({ messages: newMessages }: BackendToUserEventPayloads['messages:all']) => {
            messages.value = newMessages;
        },
    );

    connection.socket.on(
        'messages:new',
        ({ message }: BackendToUserEventPayloads['messages:new']) => {
            messages.value.push(message);
        },
    );

    return { messages };
});
