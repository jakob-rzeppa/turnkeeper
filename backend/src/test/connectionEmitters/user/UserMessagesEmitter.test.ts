import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import UserMessagesEmitter from '../../../connectionEmitters/user/UserMessagesEmitter.js';
import messageRepository from '../../../repositories/messageRepository.js';

vi.mock('../../../repositories/messageRepository', () => ({
    default: {
        getMessagesByPlayerId: vi.fn(),
    },
}));

describe('UserMessagesEmitter', () => {
    let mockSocket: Socket;
    let emitter: UserMessagesEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;
    });

    describe('constructor', () => {
        it('should call sendAllMessages on initialization', () => {
            const spy = vi
                .spyOn(UserMessagesEmitter.prototype, 'sendAllMessages')
                .mockReturnValue();

            emitter = new UserMessagesEmitter(1, mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe('sendAllMessages', () => {
        it('should emit messages:all with the correct payload', () => {
            vi.mocked(messageRepository.getMessagesByPlayerId).mockReturnValue([]);

            emitter = new UserMessagesEmitter(1, mockSocket);
            // The sendAllMessages is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            vi.mocked(messageRepository.getMessagesByPlayerId).mockReturnValue([
                {
                    content: 'Hello, Player 1!',
                    id: 1,
                    playerId: 1,
                    sendBy: 'gm',
                    timestamp: new Date('2024-01-01T10:00:00Z'),
                },
                {
                    content: 'How are you?',
                    id: 2,
                    playerId: 1,
                    sendBy: 'player',
                    timestamp: new Date('2024-01-01T10:05:00Z'),
                },
            ]);

            emitter.sendAllMessages();

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith('messages:all', {
                messages: [
                    {
                        content: 'Hello, Player 1!',
                        id: 1,
                        playerId: 1,
                        sendBy: 'gm',
                        timestamp: new Date('2024-01-01T10:00:00Z'),
                    },
                    {
                        content: 'How are you?',
                        id: 2,
                        playerId: 1,
                        sendBy: 'player',
                        timestamp: new Date('2024-01-01T10:05:00Z'),
                    },
                ],
            });
        });
    });
});
