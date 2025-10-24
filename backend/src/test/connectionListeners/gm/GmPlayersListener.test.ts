import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmPlayersListener from '../../../connectionListeners/gm/GmPlayersListener.js';
import playersHandler from '../../../services/playersHandler.js';
import { statsHandler } from '../../../services/statsHandler.js';

// Mock the service dependencies
vi.mock('../../../services/playersHandler', () => ({
    default: {
        createPlayer: vi.fn(),
        deletePlayer: vi.fn(),
        updatePlayerInfo: vi.fn(),
    },
}));

vi.mock('../../../services/statsHandler', () => ({
    statsHandler: {
        createStatForAllPlayers: vi.fn(),
        createStatForPlayer: vi.fn(),
        removeStat: vi.fn(),
        updateStatValue: vi.fn(),
    },
}));

describe('GmPlayersListener', () => {
    let mockSocket: Socket;
    let listener: GmPlayersListener;
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

        listener = new GmPlayersListener(mockSocket);
    });

    describe('constructor', () => {
        it('should create an instance', () => {
            expect(listener).toBeInstanceOf(GmPlayersListener);
        });

        it('should register all player event listeners', () => {
            expect(mockSocket.on).toHaveBeenCalledWith('players:create', expect.any(Function));
            expect(mockSocket.on).toHaveBeenCalledWith('players:update', expect.any(Function));
            expect(mockSocket.on).toHaveBeenCalledWith('players:delete', expect.any(Function));
            expect(mockSocket.on).toHaveBeenCalledWith(
                'players:stats:create',
                expect.any(Function),
            );
            expect(mockSocket.on).toHaveBeenCalledWith(
                'players:stats:update',
                expect.any(Function),
            );
            expect(mockSocket.on).toHaveBeenCalledWith(
                'players:stats:remove',
                expect.any(Function),
            );
        });
    });

    describe('players:create event', () => {
        it('should create a player with the provided data', () => {
            const playerData = { name: 'Test Player' };

            eventHandlers['players:create'](playerData);

            expect(playersHandler.createPlayer).toHaveBeenCalledWith(playerData);
            expect(playersHandler.createPlayer).toHaveBeenCalledTimes(1);
        });
    });

    describe('players:update event', () => {
        it('should update a player with the provided data and ID', () => {
            const payload = {
                playerData: { name: 'Updated Player' },
                playerId: 1,
            };

            eventHandlers['players:update'](payload);

            expect(playersHandler.updatePlayerInfo).toHaveBeenCalledWith({
                playerData: payload.playerData,
                playerId: payload.playerId,
            });
            expect(playersHandler.updatePlayerInfo).toHaveBeenCalledTimes(1);
        });
    });

    describe('players:delete event', () => {
        it('should delete a player by ID', () => {
            const payload = { playerId: 1 };

            eventHandlers['players:delete'](payload);

            expect(playersHandler.deletePlayer).toHaveBeenCalledWith(1);
            expect(playersHandler.deletePlayer).toHaveBeenCalledTimes(1);
        });
    });

    describe('players:stats:create event', () => {
        describe("when scope is 'player'", () => {
            it('should create a stat for a specific player', () => {
                const payload = {
                    playerId: 1,
                    scope: 'player' as const,
                    statData: { name: 'Strength', value: 10 },
                };

                eventHandlers['players:stats:create'](payload);

                expect(statsHandler.createStatForPlayer).toHaveBeenCalledWith({
                    playerId: payload.playerId,
                    statData: payload.statData,
                });
                expect(statsHandler.createStatForAllPlayers).not.toHaveBeenCalled();
            });

            it('should allow statData with empty value', () => {
                const payload = {
                    playerId: 1,
                    scope: 'player' as const,
                    statData: { name: 'Agility', value: '' },
                };

                eventHandlers['players:stats:create'](payload);

                expect(statsHandler.createStatForPlayer).toHaveBeenCalledWith({
                    playerId: payload.playerId,
                    statData: payload.statData,
                });
            });
        });

        describe("when scope is 'global'", () => {
            it('should create a stat for all players', () => {
                const payload = {
                    scope: 'global' as const,
                    statData: { name: 'Health', value: 100 },
                };

                eventHandlers['players:stats:create'](payload);

                expect(statsHandler.createStatForAllPlayers).toHaveBeenCalledWith(payload.statData);
                expect(statsHandler.createStatForPlayer).not.toHaveBeenCalled();
            });

            it('should allow statData with empty value', () => {
                const payload = {
                    scope: 'global' as const,
                    statData: { name: 'Stamina', value: '' },
                };

                eventHandlers['players:stats:create'](payload);

                expect(statsHandler.createStatForAllPlayers).toHaveBeenCalledWith(payload.statData);
            });
        });
    });

    describe('players:stats:update event', () => {
        it('should update a stat value for a specific player and stat ID', () => {
            const payload = { playerId: 1, statId: 3, value: 15 };

            eventHandlers['players:stats:update'](payload);

            expect(statsHandler.updateStatValue).toHaveBeenCalledWith({
                newValue: payload.value,
                playerId: payload.playerId,
                statId: payload.statId,
            });
            expect(statsHandler.updateStatValue).toHaveBeenCalledTimes(1);
        });

        it('should allow updating stat value to empty', () => {
            const payload = { playerId: 1, statId: 3, value: '' };

            eventHandlers['players:stats:update'](payload);

            expect(statsHandler.updateStatValue).toHaveBeenCalledWith({
                newValue: payload.value,
                playerId: payload.playerId,
                statId: payload.statId,
            });
        });
    });

    describe('players:stats:remove event', () => {
        it('should remove a stat by player ID and stat ID', () => {
            const payload = { playerId: 1, statId: 5 };

            eventHandlers['players:stats:remove'](payload);

            expect(statsHandler.removeStat).toHaveBeenCalledWith({
                playerId: payload.playerId,
                statId: payload.statId,
            });
            expect(statsHandler.removeStat).toHaveBeenCalledTimes(1);
        });
    });
});
