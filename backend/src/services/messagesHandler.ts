import { Message } from 'shared-types';

import GmController from '../connectionControllers/GmController.js';
import UserController from '../connectionControllers/UserController.js';
import messageRepository from '../repositories/messageRepository.js';
import logger from './logger.js';

const messagesHandler = {
    sendMessageFromPlayer: (playerId: number, content: string) => {
        const message: Omit<Message, 'id' | 'timestamp'> = {
            content,
            playerId,
            sendBy: 'player',
        };

        const createdMessage = messageRepository.createMessage(message);

        if (!createdMessage) {
            // Handle with just a log, since the players sit next to each other
            logger.warn({ message: `Failed to send message from player ${playerId}` });
            return;
        }

        GmController.getInstance()?.gmMessagesEmitter.sendNewMessage(createdMessage);
        UserController.getInstance(playerId)?.userMessagesEmitter.sendNewMessage(createdMessage);
    },
    sendMessageToPlayer: (playerId: number, content: string) => {
        const message: Omit<Message, 'id' | 'timestamp'> = {
            content,
            playerId,
            sendBy: 'gm',
        };

        const createdMessage = messageRepository.createMessage(message);

        if (!createdMessage) {
            // Handle with just a log, since the players sit next to each other
            logger.warn({ message: `Failed to send message to player ${playerId}` });
            return;
        }

        GmController.getInstance()?.gmMessagesEmitter.sendNewMessage(createdMessage);
        UserController.getInstance(playerId)?.userMessagesEmitter.sendNewMessage(createdMessage);
    },
};

export default messagesHandler;
