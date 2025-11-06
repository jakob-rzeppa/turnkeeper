import useConnection from '@/composables/useConnection';
import type { UserToBackendEventPayloads } from 'shared-types';

export const useMessagesEmitter = () => {
    const connection = useConnection();

    function sendMessage(content: string) {
        const payload: UserToBackendEventPayloads['messages:send'] = { content };

        connection.socket.emit('messages:send', payload);
    }

    return { sendMessage };
};
