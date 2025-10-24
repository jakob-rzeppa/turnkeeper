import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { usePlayerStore } from '../../stores/playerStore';

const mockSocketOn = vi.fn();
vi.mock('@/composables/connection', () => ({
    default: () => ({
        socket: {
            on: mockSocketOn,
        },
    }),
}));

describe('usePlayerStore', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        setActivePinia(createPinia());
    });

    it('initializes with null: player', () => {
        const playerStore = usePlayerStore();

        expect(playerStore.player).toBeNull();
    });

    it('registers socket listener for player:info event', () => {
        usePlayerStore();

        expect(mockSocketOn).toHaveBeenCalledWith('player:info', expect.any(Function));
    });

    describe('handling player:info event', () => {
        it('updates player correctly', () => {
            const playerStore = usePlayerStore();

            const mockPlayer = { id: 1, name: 'Alice' };

            const playerInfoHandler = mockSocketOn.mock.calls.find(
                (call) => call[0] === 'player:info',
            )?.[1];

            playerInfoHandler({ player: mockPlayer });

            expect(playerStore.player).toEqual(mockPlayer);
        });
    });
});
