import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads } from 'shared-types';
import { useMessagesStore } from '@/stores/messagesStore';
import { usePlayerStore } from '@/stores/playerStore';

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

const addToastMock = vi.fn();

vi.mock('@/stores/toastStore', () => {
    return {
        useToastStore: () => ({
            addToast: addToastMock,
        }),
    };
});

vi.mock('@/stores/playerStore', () => {
    return {
        usePlayerStore: () => ({
            getPlayerById: (id: string) => ({ id, name: `Player ${id}` }),
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

    it('registers socket listener for messages:all and messages:new events', () => {
        useMessagesStore();

        vi.mocked(socket.on).mock.calls.find((call) => call[0] === 'messages:all');
        vi.mocked(socket.on).mock.calls.find((call) => call[0] === 'messages:new');
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

    describe('handling messages:new event', () => {
        it('creates message array for new playerId if not existing', () => {
            // Initialize the store first to register the socket listener
            useMessagesStore();

            const messagesNewHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'messages:new')?.[1] as (
                payload: BackendToGmEventPayloads['messages:new'],
            ) => void;

            messagesNewHandler({
                message: {
                    id: 3,
                    playerId: 2,
                    content: 'Another new test message',
                    timestamp: new Date('2024-01-01T14:00:00Z'),
                    sendBy: 'player',
                },
            });

            expect(useMessagesStore().messages).toEqual({
                2: [
                    {
                        id: 3,
                        playerId: 2,
                        content: 'Another new test message',
                        timestamp: new Date('2024-01-01T14:00:00Z'),
                        sendBy: 'player',
                    },
                ],
            });
        });

        it('appends message to existing playerId array', () => {
            // Initialize the store first to register the socket listener
            const messagesStore = useMessagesStore();

            // Pre-populate with a message for playerId 1
            messagesStore.messages = {
                1: [
                    {
                        id: 1,
                        playerId: 1,
                        content: 'Existing message',
                        timestamp: new Date('2024-01-01T12:00:00Z'),
                        sendBy: 'gm',
                    },
                ],
            };

            const messagesNewHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'messages:new')?.[1] as (
                payload: BackendToGmEventPayloads['messages:new'],
            ) => void;

            messagesNewHandler({
                message: {
                    id: 2,
                    playerId: 1,
                    content: 'New test message',
                    timestamp: new Date('2024-01-01T13:00:00Z'),
                    sendBy: 'player',
                },
            });

            expect(useMessagesStore().messages).toEqual({
                1: [
                    {
                        id: 1,
                        playerId: 1,
                        content: 'Existing message',
                        timestamp: new Date('2024-01-01T12:00:00Z'),
                        sendBy: 'gm',
                    },
                    {
                        id: 2,
                        playerId: 1,
                        content: 'New test message',
                        timestamp: new Date('2024-01-01T13:00:00Z'),
                        sendBy: 'player',
                    },
                ],
            });
        });

        describe('toast notifications', () => {
            it('calls toastStore.addToast on new message from system', () => {
                // Initialize the store first to register the socket listener
                useMessagesStore();

                const messagesNewHandler = vi
                    .mocked(socket.on)
                    .mock.calls.find((call) => call[0] === 'messages:new')?.[1] as (
                    payload: BackendToGmEventPayloads['messages:new'],
                ) => void;

                messagesNewHandler({
                    message: {
                        id: 4,
                        playerId: 3,
                        content: 'Toast test message',
                        timestamp: new Date('2024-01-01T15:00:00Z'),
                        sendBy: 'system',
                    },
                });

                expect(addToastMock).toHaveBeenCalledWith('New message from system', 'info', 5000);
            });

            it('calls toastStore.addToast on new message from player', () => {
                // Initialize the store first to register the socket listener
                useMessagesStore();

                const messagesNewHandler = vi
                    .mocked(socket.on)
                    .mock.calls.find((call) => call[0] === 'messages:new')?.[1] as (
                    payload: BackendToGmEventPayloads['messages:new'],
                ) => void;

                messagesNewHandler({
                    message: {
                        id: 5,
                        playerId: 42,
                        content: 'Another toast test message',
                        timestamp: new Date('2024-01-01T16:00:00Z'),
                        sendBy: 'player',
                    },
                });

                expect(addToastMock).toHaveBeenCalledWith(
                    'New message from Player 42',
                    'info',
                    5000,
                );
            });

            it('does not call toastStore.addToast on new message from gm', () => {
                // Initialize the store first to register the socket listener
                useMessagesStore();

                const messagesNewHandler = vi
                    .mocked(socket.on)
                    .mock.calls.find((call) => call[0] === 'messages:new')?.[1] as (
                    payload: BackendToGmEventPayloads['messages:new'],
                ) => void;

                messagesNewHandler({
                    message: {
                        id: 6,
                        playerId: 99,
                        content: 'GM message test',
                        timestamp: new Date('2024-01-01T17:00:00Z'),
                        sendBy: 'gm',
                    },
                });

                expect(addToastMock).not.toHaveBeenCalled();
            });
        });
    });
});
