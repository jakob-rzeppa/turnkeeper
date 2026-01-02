import { Message } from '../entities/Message.js';

import GmController from '../connectionControllers/GmController.js';
import UserController from '../connectionControllers/UserController.js';
import messageRepository from '../repositories/messageRepository.js';
import logger from './logger.js';

const messagesHandler = {
    /**
     * Handles a message sent from a player, creates the message in the database
     * and notifies both the GM and the player about the new message.
     *
     * DEPENDENCIES:
     * - GmController -> gmMessagesEmitter.sendNewMessage
     * - UserController -> userMessagesEmitter.sendNewMessage
     * - messageRepository.createMessage
     * - logger.error
     *
     * @param playerId the ID of the player sending the message
     * @param content the content of the message
     */
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

    /**
     * Handles sending a message to a player from the GM,
     * creates the message in the database and notifies both the GM and the player about the new message.
     *
     * DEPENDENCIES:
     * - GmController -> gmMessagesEmitter.sendNewMessage
     * - UserController -> userMessagesEmitter.sendNewMessage
     * - messageRepository.createMessage
     * - logger.error
     *
     * @param playerId the ID of the player to send the message to
     * @param content the content of the message
     */
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
