import { Player } from '../../../entities/Player.js';
import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import UserPlayersEmitter from '../../../connectionEmitters/user/UserPlayersEmitter.js';
import playerRepository from '../../../repositories/playerRepository.js';
import { NotFound } from '../../../repositories/repositoryErrors.js';

// Mock the player repository
vi.mock('../../../repositories/playerRepository', () => ({
    default: {
        getPlayerById: vi.fn(),
    },
}));

// Mock the logger
vi.mock('../../../services/logger', () => ({
    default: {
        error: vi.fn(),
    },
}));

describe('UserPlayersEmitter', () => {
    let mockSocket: Socket;
    const playerId = 1;
    let emitter: UserPlayersEmitter;

    const mockPlayer: Player = {
        id: 1,
        name: 'Player 1',
        notes: 'Some notes',
        hiddenNotes: 'Hidden notes',
        secret: 'secret123',
        stats: [
            { id: 1, playerId: 1, name: 'HP', value: 100 },
            { id: 2, playerId: 1, name: 'AC', value: 15 },
        ],
    } as Player;

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;

        // Default mock for repository
        vi.mocked(playerRepository.getPlayerById).mockReturnValue(mockPlayer);
    });

    describe('constructor', () => {
        it('should call sendOwnPlayer on initialization', () => {
            const spy = vi.spyOn(UserPlayersEmitter.prototype, 'sendOwnPlayer');

            emitter = new UserPlayersEmitter(playerId, mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe('sendOwnPlayer', () => {
        it('should emit player:info with the correct payload', () => {
            emitter = new UserPlayersEmitter(playerId, mockSocket);
            vi.clearAllMocks();

            emitter.sendOwnPlayer();

            expect(mockSocket.emit).toHaveBeenCalledWith('player:info', {
                player: {
                    id: mockPlayer.id,
                    name: mockPlayer.name,
                    notes: mockPlayer.notes,
                    stats: mockPlayer.stats,
                },
            });
        });

        it('should not include secret or hiddenNotes in emitted player data', () => {
            emitter = new UserPlayersEmitter(playerId, mockSocket);
            vi.clearAllMocks();

            emitter.sendOwnPlayer();

            const emittedPayload = vi.mocked(mockSocket.emit).mock.calls[0][1];
            expect(emittedPayload.player).not.toHaveProperty('secret');
            expect(emittedPayload.player).not.toHaveProperty('hiddenNotes');
        });

        it('should handle NotFound error gracefully', () => {
            vi.mocked(playerRepository.getPlayerById).mockImplementation(() => {
                throw new NotFound('Player not found');
            });

            emitter = new UserPlayersEmitter(playerId, mockSocket);
            vi.clearAllMocks();

            emitter.sendOwnPlayer();

            expect(mockSocket.emit).not.toHaveBeenCalled();
        });

        it('should handle generic errors gracefully', () => {
            vi.mocked(playerRepository.getPlayerById).mockImplementation(() => {
                throw new Error('Database error');
            });

            emitter = new UserPlayersEmitter(playerId, mockSocket);
            vi.clearAllMocks();

            emitter.sendOwnPlayer();

            expect(mockSocket.emit).not.toHaveBeenCalled();
        });
    });
});
