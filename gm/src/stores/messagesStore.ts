import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToGmEventPayloads, Message } from 'shared-types';
import { ref } from 'vue';
import { useToastStore } from './toastStore';

const connection = useConnection();

export const useMessagesStore = defineStore('messages', () => {
    const messages = ref<{ [key: number]: Message[] }>({});
    const toastStore = useToastStore();

    connection.socket.on(
        'messages:all',
        ({ messages: newMessages }: BackendToGmEventPayloads['messages:all']) => {
            toastStore.addToast('New messages received', 'info', 4000);

            messages.value = newMessages;
        },
    );

    return { messages };
});
