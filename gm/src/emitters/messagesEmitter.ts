import useConnection from '@/composables/useConnection';
import type { GmToBackendEventPayloads } from 'shared-types';

export const useMessagesEmitter = () => {
    const connection = useConnection();

    function sendMessage(playerId: number, content: string) {
        const payload: GmToBackendEventPayloads['messages:send'] = { playerId, content };

        connection.socket.emit('messages:send', payload);
    }

    return { sendMessage };
};
