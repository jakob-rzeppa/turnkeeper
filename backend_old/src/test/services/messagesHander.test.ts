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
                    sendNewMessage: vi.fn(),
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
                    sendNewMessage: vi.fn(),
                },
            }),
        },
    };
});

describe('messagesHandler service', () => {
    describe('sendMessageFromPlayer', () => {
        it('should call messageRepository with the message from a player', () => {
            vi.mocked(messageRepository.createMessage).mockReturnValue({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: new Date(),
            });

            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(messageRepository.createMessage).toHaveBeenCalledTimes(1);
            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
            });
        });

        it('should call gmMessagesEmitter.sendNewMessage and userMessagesEmitter.sendNewMessage with the created message', () => {
            vi.mocked(messageRepository.createMessage).mockReturnValue({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: new Date(),
            });

            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(
                GmController.getInstance()?.gmMessagesEmitter.sendNewMessage,
            ).toHaveBeenCalledWith({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: expect.any(Date),
            });
            expect(
                UserController.getInstance(1)?.userMessagesEmitter.sendNewMessage,
            ).toHaveBeenCalledWith({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: expect.any(Date),
            });
        });
    });

    describe('sendMessageToPlayer', () => {
        it('should call messageRepository with the message to a player', () => {
            vi.mocked(messageRepository.createMessage).mockReturnValue({
                id: 1,
                content: 'Hello to player',
                playerId: 2,
                sendBy: 'gm',
                timestamp: new Date(),
            });

            messagesHandler.sendMessageToPlayer(2, 'Hello to player');

            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                content: 'Hello to player',
                playerId: 2,
                sendBy: 'gm',
            });
        });

        it('should call gmMessagesEmitter.sendAllMessages and userMessagesEmitter.sendAllMessages', () => {
            vi.mocked(messageRepository.createMessage).mockReturnValue({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: new Date(),
            });

            messagesHandler.sendMessageFromPlayer(1, 'Hello from player');

            expect(
                GmController.getInstance()?.gmMessagesEmitter.sendNewMessage,
            ).toHaveBeenCalledWith({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: expect.any(Date),
            });
            expect(
                UserController.getInstance(1)?.userMessagesEmitter.sendNewMessage,
            ).toHaveBeenCalledWith({
                id: 1,
                content: 'Hello from player',
                playerId: 1,
                sendBy: 'player',
                timestamp: expect.any(Date),
            });
        });
    });
});
