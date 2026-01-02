import { BackendToUserEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import messageRepository from '../../repositories/messageRepository.js';
import { Message } from '../../entities/Message.js';

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
        let messages: Message[] = [];
        try {
            messages = messageRepository.getMessagesByPlayerId(this.playerId);
        } catch (err: unknown) {
            // If there's an error (e.g., player not found), we simply send an empty array
            messages = [];
        }

        const payload: BackendToUserEventPayloads['messages:all'] = {
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

    public sendNewMessage(message: Message) {
        const payload: BackendToUserEventPayloads['messages:new'] = {
            message,
        };

        this.socket.emit('messages:new', payload);
    }
}
