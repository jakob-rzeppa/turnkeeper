import { BackendToUserPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import messageRepository from '../../repositories/messageRepository.js';

export default class UserMessagesEmitter {
    private playerId: number;
    private socket: Socket;

    public constructor(playerId: number, s: Socket) {
        this.playerId = playerId;
        this.socket = s;

        // Initial emits
        this.sendAllMessages();
    }

    public sendAllMessages() {
        const messages = messageRepository.getMessagesByPlayerId(this.playerId);

        const payload: BackendToUserPayloads['messages:all'] = {
            messages: messages.map((msg) => ({
                content: msg.content,
                id: msg.id,
                playerId: msg.playerId,
                sendBy: msg.sendBy,
                timestamp: msg.timestamp,
            })),
        };

        this.socket.emit('messages:all', payload);
    }
}
