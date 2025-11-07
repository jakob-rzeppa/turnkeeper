import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import useConnection from '@/composables/useConnection';
import type { BackendToGmEventPayloads } from 'shared-types';
import { useGameStore } from '@/stores/gameStore';

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

describe('useGameStore', () => {
    const connection = useConnection();
    const socket = connection.socket;

    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with default values', () => {
        const gameStore = useGameStore();

        expect(gameStore.playerOrder).toEqual([]);
        expect(gameStore.isInitialized).toBe(false);
        expect(gameStore.round).toEqual({
            roundNumber: 0,
            currentPlayerIndex: 0,
        });
        expect(gameStore.notes).toBe('');
        expect(gameStore.hiddenNotes).toBe('');
        expect(gameStore.currentPlayerId).toBeNull();
    });

    it('registers socket listener for game:info event', () => {
        useGameStore();

        expect(socket.on).toHaveBeenCalledWith('game:info', expect.any(Function));
    });

    describe('handling game:info event', () => {
        it('updates game state correctly when gameState is provided', () => {
            // Initialize the store first to register the socket listener
            useGameStore();

            const gameInfoHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'game:info')?.[1] as (
                payload: BackendToGmEventPayloads['game:info'],
            ) => void;

            gameInfoHandler({
                gameState: {
                    id: 1,
                    playerOrder: [
                        { id: 1, name: 'Player 1' },
                        { id: 2, name: 'Player 2' },
                    ],
                    currentPlayerIndex: 1,
                    roundNumber: 3,
                    notes: 'Game notes',
                    hiddenNotes: 'Hidden notes',
                },
            });

            const gameStore = useGameStore();
            expect(gameStore.playerOrder).toEqual([
                { id: 1, name: 'Player 1' },
                { id: 2, name: 'Player 2' },
            ]);
            expect(gameStore.isInitialized).toBe(true);
            expect(gameStore.round).toEqual({
                roundNumber: 3,
                currentPlayerIndex: 1,
            });
            expect(gameStore.notes).toBe('Game notes');
            expect(gameStore.hiddenNotes).toBe('Hidden notes');
            expect(gameStore.currentPlayerId).toBe(2);
        });

        it('sets isInitialized to false when gameState is null', () => {
            // Initialize the store first to register the socket listener
            useGameStore();

            const gameInfoHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'game:info')?.[1] as (
                payload: BackendToGmEventPayloads['game:info'],
            ) => void;

            gameInfoHandler({
                gameState: null,
            });

            const gameStore = useGameStore();
            expect(gameStore.isInitialized).toBe(false);
        });

        it('computes currentPlayerId correctly', () => {
            useGameStore();

            const gameInfoHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'game:info')?.[1] as (
                payload: BackendToGmEventPayloads['game:info'],
            ) => void;

            gameInfoHandler({
                gameState: {
                    id: 2,
                    playerOrder: [
                        { id: 10, name: 'Player A' },
                        { id: 20, name: 'Player B' },
                        { id: 30, name: 'Player C' },
                    ],
                    currentPlayerIndex: 2,
                    roundNumber: 1,
                    notes: '',
                    hiddenNotes: '',
                },
            });

            const gameStore = useGameStore();
            expect(gameStore.currentPlayerId).toBe(30);
        });

        it('returns null for currentPlayerId when playerOrder is empty', () => {
            useGameStore();

            const gameInfoHandler = vi
                .mocked(socket.on)
                .mock.calls.find((call) => call[0] === 'game:info')?.[1] as (
                payload: BackendToGmEventPayloads['game:info'],
            ) => void;

            gameInfoHandler({
                gameState: {
                    id: 3,
                    playerOrder: [],
                    currentPlayerIndex: 0,
                    roundNumber: 1,
                    notes: '',
                    hiddenNotes: '',
                },
            });

            const gameStore = useGameStore();
            expect(gameStore.currentPlayerId).toBeNull();
        });
    });
});
