import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads } from 'shared-types';
import { useLogStore } from '@/stores/logStore';

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

describe('useLogStore', () => {
    const connection = useConnection();
    const socket = connection.socket;

    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with null: player', () => {
        const logStore = useLogStore();

        expect(logStore.logs).toHaveLength(0);
    });

    it('registers socket listener for players:info event', () => {
        useLogStore();

        vi.mocked(socket.on).mock.calls.find((call) => call[0] === 'players:info');
    });

    describe('handling players:info event', () => {
        it('updates players correctly', () => {
            // Initialize the store first to register the socket listener
            useLogStore();

            const logEntryHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'log:entry')?.[1] as (
                payload: BackendToGmEventPayloads['log:entry'],
            ) => void;

            logEntryHandler({
                entry: {
                    timestamp: new Date('2024-01-01T12:00:00Z'),
                    severity: 'info',
                    message: 'Test log entry',
                },
            });

            expect(useLogStore().logs).toEqual([
                {
                    timestamp: new Date('2024-01-01T12:00:00Z'),
                    severity: 'info',
                    message: 'Test log entry',
                },
            ]);
        });
    });
});
