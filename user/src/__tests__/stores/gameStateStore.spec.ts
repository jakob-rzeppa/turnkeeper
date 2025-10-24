import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useGameStateStore } from '../../stores/gameStateStore';

const mockSocketOn = vi.fn();
vi.mock('@/composables/connection', () => ({
    default: () => ({
        socket: {
            on: mockSocketOn,
        },
    }),
}));

describe('useGameStateStore', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with null: gameState and currentPlayer', () => {
        const gameStateStore = useGameStateStore();

        expect(gameStateStore.gameState).toBeNull();
        expect(gameStateStore.currentPlayer).toBeNull();
    });

    it('registers socket listener for game:info event', () => {
        useGameStateStore();

        expect(mockSocketOn).toHaveBeenCalledWith('game:info', expect.any(Function));
    });

    describe('handling game:info event', () => {
        it('updates gameState and currentPlayer correctly', () => {
            const gameStateStore = useGameStateStore();

            const mockGameState = {
                playerOrder: [
                    { id: 1, name: 'Alice' },
                    { id: 2, name: 'Bob' },
                ],
                currentPlayerIndex: 0,
            };

            const gameInfoHandler = mockSocketOn.mock.calls.find(
                (call) => call[0] === 'game:info',
            )?.[1];

            gameInfoHandler({ gameState: mockGameState });

            expect(gameStateStore.gameState).toEqual(mockGameState);
            expect(gameStateStore.currentPlayer).toEqual({ id: 1, name: 'Alice' });
        });
    });
});
