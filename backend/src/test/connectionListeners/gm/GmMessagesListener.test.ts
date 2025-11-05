import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmMessagesListener from '../../../connectionListeners/gm/GmMessagesListener.js';
import messagesHandler from '../../../services/messagesHandler.js';

vi.mock('../../../services/messagesHandler.js', () => ({
    default: {
        sendMessageToPlayer: vi.fn(),
    },
}));

describe('GmMessagesListener', () => {
    let mockSocket: Socket;
    let listener: GmMessagesListener;
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

        listener = new GmMessagesListener(mockSocket);
    });

    describe('constructor', () => {
        it('should create an instance', () => {
            expect(listener).toBeInstanceOf(GmMessagesListener);
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

            expect(messagesHandler.sendMessageToPlayer).toHaveBeenCalledWith(
                testPayload.playerId,
                testPayload.content,
            );
        });
    });
});
