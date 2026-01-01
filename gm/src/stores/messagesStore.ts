import useConnection from '@/composables/useConnection';
import { defineStore } from 'pinia';
import type { BackendToGmEventPayloads, Message } from 'shared-types';
import { ref } from 'vue';
import { useToastStore } from './toastStore';
import { usePlayerStore } from './playerStore';

const connection = useConnection();

export const useMessagesStore = defineStore('messages', () => {
    const messages = ref<{ [key: number]: Message[] }>({});
    const toastStore = useToastStore();
    const playerStore = usePlayerStore();

    connection.socket.on(
        'messages:all',
        ({ messages: newMessages }: BackendToGmEventPayloads['messages:all']) => {
            messages.value = newMessages;
        },
    );

    connection.socket.on(
        'messages:new',
        ({ message }: BackendToGmEventPayloads['messages:new']) => {
            if (message.sendBy === 'system') {
                toastStore.addToast('New message from system', 'info', 5000);
            } else if (message.sendBy === 'player') {
                toastStore.addToast(
                    'New message from ' + playerStore.getPlayerById(message.playerId)?.name ||
                        'a player',
                    'info',
                    5000,
                );
            }

            if (!messages.value[message.playerId]) {
                messages.value[message.playerId] = [];
            }
            messages.value[message.playerId].push(message);
        },
    );

    return { messages };
});
