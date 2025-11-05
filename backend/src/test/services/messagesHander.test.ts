import { describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import UserController from '../../connectionControllers/UserController.js';
import messageRepository from '../../repositories/messageRepository.js';
import messagesHandler from '../../services/messagesHandler.js';

vi.mock('../../repositories/messageRepository', () => {
    return {
        default: {
            createMessage: vi.fn(),
        },
    };
});

vi.mock('../../connectionControllers/GmController', () => {
    return {
        default: {
            getInstance: vi.fn().mockReturnValue({
                gmMessagesEmitter: {
                    sendAllMessages: vi.fn(),
                },
            }),
        },
    };
});

vi.mock('../../connectionControllers/UserController', () => {
    return {
        default: {
            getInstance: vi.fn().mockReturnValue({
                userMessagesEmitter: {
                    sendAllMessages: vi.fn(),
                },
            }),
        },
    };
});

describe('messagesHandler service', () => {
    describe('sendMessageFromPlayer', () => {
        it('should send a message from a player', () => {
            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
            });
        });

        it('should call gmMessagesEmitter.sendAllMessages and userMessagesEmitter.sendAllMessages', () => {
            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(
                GmController.getInstance()?.gmMessagesEmitter.sendAllMessages,
            ).toHaveBeenCalled();
            expect(
                UserController.getInstance(1)?.userMessagesEmitter.sendAllMessages,
            ).toHaveBeenCalled();
        });
    });

    describe('sendMessageToPlayer', () => {
        it('should send a message to a player', () => {
            messagesHandler.sendMessageToPlayer(2, 'Hello to player');

            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                content: 'Hello to player',
                playerId: 2,
                sendBy: 'gm',
            });
        });

        it('should call gmMessagesEmitter.sendAllMessages and userMessagesEmitter.sendAllMessages', () => {
            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(
                GmController.getInstance()?.gmMessagesEmitter.sendAllMessages,
            ).toHaveBeenCalled();
            expect(
                UserController.getInstance(1)?.userMessagesEmitter.sendAllMessages,
            ).toHaveBeenCalled();
        });
    });
});
