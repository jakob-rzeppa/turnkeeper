import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import UserMessagesListener from '../../../connectionListeners/user/UserMessagesListener.js';
import messagesHandler from '../../../services/messagesHandler.js';

vi.mock('../../../services/messagesHandler.js', () => ({
    default: {
        sendMessageFromPlayer: vi.fn(),
    },
}));

describe('UserMessagesListener', () => {
    let mockSocket: Socket;
    let listener: UserMessagesListener;
    let eventHandlers: Record<string, (...args: unknown[]) => void>;

    beforeEach(() => {
        vi.clearAllMocks();
        eventHandlers = {};

        // Create a mock socket that captures event handlers
        mockSocket = {
            id: 'mock-socket-id',
            on: vi.fn((event: string, handler: (...args: unknown[]) => void) => {
                eventHandlers[event] = handler;
            }),
        } as unknown as Socket;

        listener = new UserMessagesListener(1, mockSocket);
    });

    describe('constructor', () => {
        it('should create an instance', () => {
            expect(listener).toBeInstanceOf(UserMessagesListener);
        });

        it('should register all messages event listeners', () => {
            expect(mockSocket.on).toHaveBeenCalledWith('messages:send', expect.any(Function));
        });
    });

    describe('messages:send event', () => {
        it('should call sendMessage with correct parameters', () => {
            const testPayload = { playerId: 1, content: 'Test message' };

            // Simulate the event being triggered
            eventHandlers['messages:send'](testPayload);

            expect(messagesHandler.sendMessageFromPlayer).toHaveBeenCalledWith(
                testPayload.playerId,
                testPayload.content,
            );
        });
    });
});
