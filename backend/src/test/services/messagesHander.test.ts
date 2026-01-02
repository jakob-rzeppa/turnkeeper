import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import UserController from '../../connectionControllers/UserController.js';
import messageRepository from '../../repositories/messageRepository.js';
import logger from '../../services/logger.js';
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

vi.mock('../../services/logger', () => {
    return {
        default: {
            error: vi.fn(),
        },
    };
});

describe('messagesHandler service', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe('sendMessageFromPlayer', () => {
        it('should create a message with sendBy set to "player"', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'player' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);

            messagesHandler.sendMessageFromPlayer(playerId, content);

            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                playerId,
                content,
                sendBy: 'player',
            });
        });

        it('should notify GM about the new message', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'player' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            const mockGmInstance = GmController.getInstance();

            messagesHandler.sendMessageFromPlayer(playerId, content);

            expect(mockGmInstance?.gmMessagesEmitter.sendNewMessage).toHaveBeenCalledWith(
                mockCreatedMessage,
            );
        });

        it('should notify the player about the new message', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'player' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            const mockUserInstance = UserController.getInstance(playerId);

            messagesHandler.sendMessageFromPlayer(playerId, content);

            expect(mockUserInstance?.userMessagesEmitter.sendNewMessage).toHaveBeenCalledWith(
                mockCreatedMessage,
            );
        });

        it('should handle when GM is not connected', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'player' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            vi.mocked(GmController.getInstance).mockReturnValue(null);

            expect(() => messagesHandler.sendMessageFromPlayer(playerId, content)).not.toThrow();
        });

        it('should handle when user is not connected', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'player' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            vi.mocked(UserController.getInstance).mockReturnValue(undefined);

            expect(() => messagesHandler.sendMessageFromPlayer(playerId, content)).not.toThrow();
        });

        it('should log error if message creation fails', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const error = new Error('Database error');

            vi.mocked(messageRepository.createMessage).mockImplementation(() => {
                throw error;
            });

            messagesHandler.sendMessageFromPlayer(playerId, content);

            expect(logger.error).toHaveBeenCalledWith({
                message: `Failed to handle message from player ${playerId}: ${error.message}`,
            });
        });

        it('should not notify anyone if message creation fails', () => {
            const playerId = 1;
            const content = 'Hello from player';
            const gmSendSpy = vi.fn();
            const userSendSpy = vi.fn();

            vi.mocked(GmController.getInstance).mockReturnValue({
                gmMessagesEmitter: {
                    sendNewMessage: gmSendSpy,
                },
            } as any);
            vi.mocked(UserController.getInstance).mockReturnValue({
                userMessagesEmitter: {
                    sendNewMessage: userSendSpy,
                },
            } as any);

            vi.mocked(messageRepository.createMessage).mockImplementation(() => {
                throw new Error('Database error');
            });

            messagesHandler.sendMessageFromPlayer(playerId, content);

            expect(gmSendSpy).not.toHaveBeenCalled();
            expect(userSendSpy).not.toHaveBeenCalled();
        });
    });

    describe('sendMessageToPlayer', () => {
        it('should create a message with sendBy set to "gm"', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(messageRepository.createMessage).toHaveBeenCalledWith({
                playerId,
                content,
                sendBy: 'gm',
            });
        });

        it('should notify GM about the new message', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            const mockGmInstance = GmController.getInstance();

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(mockGmInstance?.gmMessagesEmitter.sendNewMessage).toHaveBeenCalledWith(
                mockCreatedMessage,
            );
        });

        it('should notify the player about the new message', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            const mockUserInstance = UserController.getInstance(playerId);

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(mockUserInstance?.userMessagesEmitter.sendNewMessage).toHaveBeenCalledWith(
                mockCreatedMessage,
            );
        });

        it('should handle when GM is not connected', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            vi.mocked(GmController.getInstance).mockReturnValue(null);

            expect(() => messagesHandler.sendMessageToPlayer(playerId, content)).not.toThrow();
        });

        it('should handle when user is not connected', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const mockCreatedMessage = {
                id: 1,
                playerId,
                content,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            vi.mocked(messageRepository.createMessage).mockReturnValue(mockCreatedMessage);
            vi.mocked(UserController.getInstance).mockReturnValue(undefined);

            expect(() => messagesHandler.sendMessageToPlayer(playerId, content)).not.toThrow();
        });

        it('should log error if message creation fails', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const error = new Error('Database error');

            vi.mocked(messageRepository.createMessage).mockImplementation(() => {
                throw error;
            });

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(logger.error).toHaveBeenCalledWith({
                message: `Failed to handle message to player ${playerId}: ${error.message}`,
            });
        });

        it('should not notify anyone if message creation fails', () => {
            const playerId = 1;
            const content = 'Hello from GM';
            const gmSendSpy = vi.fn();
            const userSendSpy = vi.fn();

            vi.mocked(GmController.getInstance).mockReturnValue({
                gmMessagesEmitter: {
                    sendNewMessage: gmSendSpy,
                },
            } as any);
            vi.mocked(UserController.getInstance).mockReturnValue({
                userMessagesEmitter: {
                    sendNewMessage: userSendSpy,
                },
            } as any);

            vi.mocked(messageRepository.createMessage).mockImplementation(() => {
                throw new Error('Database error');
            });

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(gmSendSpy).not.toHaveBeenCalled();
            expect(userSendSpy).not.toHaveBeenCalled();
        });

        it('should log error with "Unknown error" if error is not an Error instance', () => {
            const playerId = 1;
            const content = 'Hello from GM';

            vi.mocked(messageRepository.createMessage).mockImplementation(() => {
                throw 'String error';
            });

            messagesHandler.sendMessageToPlayer(playerId, content);

            expect(logger.error).toHaveBeenCalledWith({
                message: `Failed to handle message to player ${playerId}: Unknown error`,
            });
        });
    });
});
