import { Message } from 'shared-types';

import GmController from '../connectionControllers/GmController.js';
import UserController from '../connectionControllers/UserController.js';
import messageRepository from '../repositories/messageRepository.js';

const messagesHandler = {
    sendMessageFromPlayer: (playerId: number, content: string) => {
        const message: Omit<Message, 'id' | 'timestamp'> = {
            content,
            playerId,
            sendBy: 'player',
        };

        messageRepository.createMessage(message);

        GmController.getInstance()?.gmMessagesEmitter.sendAllMessages();
        UserController.getInstance(playerId)?.userMessagesEmitter.sendAllMessages();
    },
    sendMessageToPlayer: (playerId: number, content: string) => {
        const message: Omit<Message, 'id' | 'timestamp'> = {
            content,
            playerId,
            sendBy: 'gm',
        };

        messageRepository.createMessage(message);

        GmController.getInstance()?.gmMessagesEmitter.sendAllMessages();
        UserController.getInstance(playerId)?.userMessagesEmitter.sendAllMessages();
    },
};

export default messagesHandler;
