import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads } from 'shared-types';
import { useMessagesStore } from '@/stores/messagesStore';

vi.mock('@/composables/useConnection', () => {
    const socket = {
        on: vi.fn(),
    };
    return {
        default: () => ({
            socket,
        }),
    };
});

describe('useMessagesStore', () => {
    const connection = useConnection();
    const socket = connection.socket;

    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with empty object', () => {
        const messagesStore = useMessagesStore();

        expect(messagesStore.messages).toEqual({});
    });

    it('registers socket listener for messages:all event', () => {
        useMessagesStore();

        vi.mocked(socket.on).mock.calls.find((call) => call[0] === 'messages:all');
    });

    describe('handling messages:all event', () => {
        it('updates messages correctly', () => {
            // Initialize the store first to register the socket listener
            useMessagesStore();

            const messagesAllHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'messages:all')?.[1] as (
                payload: BackendToGmEventPayloads['messages:all'],
            ) => void;

            messagesAllHandler({
                messages: {
                    1: [
                        {
                            id: 1,
                            playerId: 1,
                            content: 'Test message 1',
                            timestamp: new Date('2024-01-01T12:00:00Z'),
                            sendBy: 'player',
                        },
                    ],
                    2: [
                        {
                            id: 1,
                            playerId: 1,
                            content: 'Test message 1',
                            timestamp: new Date('2024-01-01T12:00:00Z'),
                            sendBy: 'gm',
                        },
                    ],
                },
            });

            expect(useMessagesStore().messages).toEqual({
                1: [
                    {
                        id: 1,
                        playerId: 1,
                        content: 'Test message 1',
                        timestamp: new Date('2024-01-01T12:00:00Z'),
                        sendBy: 'player',
                    },
                ],
                2: [
                    {
                        id: 1,
                        playerId: 1,
                        content: 'Test message 1',
                        timestamp: new Date('2024-01-01T12:00:00Z'),
                        sendBy: 'gm',
                    },
                ],
            });
        });
    });
});
