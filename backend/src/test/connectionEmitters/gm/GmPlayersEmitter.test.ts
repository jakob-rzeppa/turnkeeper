import { Player } from '../../../entities/Player.js';
import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmPlayersEmitter from '../../../connectionEmitters/gm/GmPlayersEmitter.js';
import playerRepository from '../../../repositories/playerRepository.js';
import { DatabaseError } from '../../../repositories/repositoryErrors.js';

// Mock the player repository
vi.mock('../../../repositories/playerRepository', () => ({
    default: {
        getAllPlayers: vi.fn(),
    },
}));

// Mock the logger
vi.mock('../../../services/logger', () => ({
    default: {
        error: vi.fn(),
    },
}));

describe('GmPlayersEmitter', () => {
    let mockSocket: Socket;
    let emitter: GmPlayersEmitter;

    const mockPlayers: Player[] = [
        {
            id: 1,
            name: 'Player 1',
            notes: 'Notes 1',
            hiddenNotes: 'Hidden 1',
            secret: 'secret1',
            stats: [{ id: 1, playerId: 1, name: 'HP', value: 100 }],
        },
        {
            id: 2,
            name: 'Player 2',
            notes: 'Notes 2',
            hiddenNotes: 'Hidden 2',
            secret: 'secret2',
            stats: [{ id: 2, playerId: 2, name: 'AC', value: 15 }],
        },
    ] as Player[];

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;

        // Default mock for repository
        vi.mocked(playerRepository.getAllPlayers).mockReturnValue(mockPlayers);
    });

    describe('constructor', () => {
        it('should call sendPlayers on initialization', () => {
            const spy = vi.spyOn(GmPlayersEmitter.prototype, 'sendPlayers');

            emitter = new GmPlayersEmitter(mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe('sendPlayers', () => {
        it('should emit players:info with the correct payload', () => {
            emitter = new GmPlayersEmitter(mockSocket);
            vi.clearAllMocks();

            emitter.sendPlayers();

            expect(mockSocket.emit).toHaveBeenCalledWith('players:info', {
                players: mockPlayers,
            });
        });

        it('should handle DatabaseError gracefully', () => {
            vi.mocked(playerRepository.getAllPlayers).mockImplementation(() => {
                throw new DatabaseError('Database connection failed');
            });

            emitter = new GmPlayersEmitter(mockSocket);
            vi.clearAllMocks();

            emitter.sendPlayers();

            expect(mockSocket.emit).not.toHaveBeenCalled();
        });

        it('should handle generic errors gracefully', () => {
            vi.mocked(playerRepository.getAllPlayers).mockImplementation(() => {
                throw new Error('Unexpected error');
            });

            emitter = new GmPlayersEmitter(mockSocket);
            vi.clearAllMocks();

            emitter.sendPlayers();

            expect(mockSocket.emit).not.toHaveBeenCalled();
        });
    });
});
