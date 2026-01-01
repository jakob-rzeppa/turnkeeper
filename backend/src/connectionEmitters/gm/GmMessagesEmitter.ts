import { BackendToGmEventPayloads, Message } from 'shared-types';
import { Socket } from 'socket.io';

import messageRepository from '../../repositories/messageRepository.js';

export default class GmMessagesEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendAllMessages();
    }

    public sendAllMessages() {
        const messagesGroupedByPlayerId = messageRepository.getAllMessagesGroupedByPlayerId();

        const payload: BackendToGmEventPayloads['messages:all'] = {
            messages: messagesGroupedByPlayerId,
        };

        this.socket.emit('messages:all', payload);
    }

    public sendNewMessage(message: Message) {
        const payload: BackendToGmEventPayloads['messages:new'] = {
            message,
        };

        this.socket.emit('messages:new', payload);
    }
}
