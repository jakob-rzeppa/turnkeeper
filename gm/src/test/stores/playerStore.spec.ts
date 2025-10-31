import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { usePlayerStore } from '@/stores/playerStore';
import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads } from 'shared-types';

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

describe('usePlayerStore', () => {
    const connection = useConnection();
    const socket = connection.socket;

    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with null: player', () => {
        const playerStore = usePlayerStore();

        expect(playerStore.players).toHaveLength(0);
    });

    it('registers socket listener for players:info event', () => {
        usePlayerStore();

        vi.mocked(socket.on).mock.calls.find((call) => call[0] === 'players:info');
    });

    describe('handling players:info event', () => {
        it('updates players correctly', () => {
            // Initialize the store first to register the socket listener
            usePlayerStore();

            const playerInfoHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'players:info')?.[1] as (
                payload: BackendToGmEventPayloads['players:info'],
            ) => void;

            playerInfoHandler({
                players: [
                    {
                        id: 1,
                        name: 'Alice',
                        secret: 'abc123',
                        notes: 'Loves chess',
                        hiddenNotes: '',
                        stats: [
                            { id: 1, name: 'gamesPlayed', value: 10 },
                            { id: 2, name: 'wins', value: 5 },
                        ],
                    },
                ],
            });

            expect(usePlayerStore().players).toEqual([
                {
                    id: 1,
                    name: 'Alice',
                    secret: 'abc123',
                    notes: 'Loves chess',
                    hiddenNotes: '',
                    stats: [
                        { id: 1, name: 'gamesPlayed', value: 10 },
                        { id: 2, name: 'wins', value: 5 },
                    ],
                },
            ]);
        });
    });
});
