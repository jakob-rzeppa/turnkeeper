import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmMessagesEmitter from '../../../connectionEmitters/gm/GmMessagesEmitter.js';
import messageRepository from '../../../repositories/messageRepository.js';

vi.mock('../../../repositories/messageRepository', () => ({
    default: {
        getAllMessagesGroupedByPlayerId: vi.fn(),
    },
}));

describe('GmMessagesEmitter', () => {
    let mockSocket: Socket;
    let emitter: GmMessagesEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;
    });

    describe('constructor', () => {
        it('should call sendAllMessages on initialization', () => {
            const spy = vi.spyOn(GmMessagesEmitter.prototype, 'sendAllMessages').mockReturnValue();

            emitter = new GmMessagesEmitter(mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe('sendAllMessages', () => {
        it('should emit messages:all with the correct payload', () => {
            vi.mocked(messageRepository.getAllMessagesGroupedByPlayerId).mockReturnValue({
                1: [
                    {
                        content: 'Hello Player 1',
                        id: 1,
                        playerId: 1,
                        sendBy: 'gm',
                        timestamp: new Date(),
                    },
                ],
                2: [
                    {
                        content: 'Hello Player 2',
                        id: 2,
                        playerId: 1,
                        sendBy: 'gm',
                        timestamp: new Date(),
                    },
                ],
            });

            emitter = new GmMessagesEmitter(mockSocket);
            // The sendGameInfo is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            emitter.sendAllMessages();

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith('messages:all', {
                messages: {
                    1: [
                        {
                            content: 'Hello Player 1',
                            id: 1,
                            playerId: 1,
                            sendBy: 'gm',
                            timestamp: expect.any(Date) as unknown as Date,
                        },
                    ],
                    2: [
                        {
                            content: 'Hello Player 2',
                            id: 2,
                            playerId: 1,
                            sendBy: 'gm',
                            timestamp: expect.any(Date) as unknown as Date,
                        },
                    ],
                },
            });
        });

        it('should emit messages:new with the correct payload', () => {
            const newMessage = {
                content: 'New message content',
                id: 3,
                playerId: 1,
                sendBy: 'gm' as const,
                timestamp: new Date(),
            };

            emitter = new GmMessagesEmitter(mockSocket);
            // The sendGameInfo is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            emitter.sendNewMessage(newMessage);

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith('messages:new', {
                message: newMessage,
            });
        });
    });
});
