import { Player } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmPlayersEmitter from '../../../connectionEmitters/gm/GmPlayersEmitter.js';
import playerRepository from '../../../repositories/playerRepository.js';

// Mock the player repository
vi.mock('../../../repositories/playerRepository', () => ({
    default: {
        getAllPlayers: vi.fn(),
    },
}));

describe('GmPlayersEmitter', () => {
    let mockSocket: Socket;
    let emitter: GmPlayersEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;
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
            const mockPlayers = [
                { id: '1', name: 'Player 1' },
                { id: '2', name: 'Player 2' },
            ];
            vi.mocked(playerRepository.getAllPlayers).mockReturnValueOnce(
                mockPlayers as unknown as Player[],
            );

            emitter = new GmPlayersEmitter(mockSocket);
            emitter.sendPlayers();

            expect(mockSocket.emit).toHaveBeenCalledWith('players:info', {
                players: mockPlayers,
            });
        });
    });
});
