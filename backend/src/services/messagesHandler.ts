import { Message } from '../entities/Message.js';

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

        try {
            const createdMessage = messageRepository.createMessage(message);

            GmController.getInstance()?.gmMessagesEmitter.sendNewMessage(createdMessage);
            UserController.getInstance(playerId)?.userMessagesEmitter.sendNewMessage(
                createdMessage,
            );
        } catch (err: unknown) {
            logger.error({
                message: `Failed to handle message from player ${playerId}: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },
    sendMessageToPlayer: (playerId: number, content: string) => {
        const message: Omit<Message, 'id' | 'timestamp'> = {
            content,
            playerId,
            sendBy: 'gm',
        };

        try {
            const createdMessage = messageRepository.createMessage(message);

            GmController.getInstance()?.gmMessagesEmitter.sendNewMessage(createdMessage);
            UserController.getInstance(playerId)?.userMessagesEmitter.sendNewMessage(
                createdMessage,
            );
        } catch (err: unknown) {
            logger.error({
                message: `Failed to handle message to player ${playerId}: ${
                    err instanceof Error ? err.message : 'Unknown error'
                }`,
            });
        }
    },
};

export default messagesHandler;
