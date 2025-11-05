import { Player } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import UserPlayersEmitter from '../../../connectionEmitters/user/UserPlayersEmitter.js';
import playerRepository from '../../../repositories/playerRepository.js';

// Mock the player repository
vi.mock('../../../repositories/playerRepository', () => ({
    default: {
        getPlayerById: vi.fn(),
    },
}));

describe('UserPlayersEmitter', () => {
    let mockSocket: Socket;
    const playerId = 1;
    let emitter: UserPlayersEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;
    });

    describe('constructor', () => {
        it('should call sendPlayers on initialization', () => {
            const spy = vi.spyOn(UserPlayersEmitter.prototype, 'sendOwnPlayer');

            emitter = new UserPlayersEmitter(playerId, mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe('sendPlayers', () => {
        it('should emit players:info with the correct payload', () => {
            emitter = new UserPlayersEmitter(playerId, mockSocket);
            // The sendOwnPlayer is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            const mockPlayer = { id: '1', name: 'Player 1' };
            vi.mocked(playerRepository.getPlayerById).mockReturnValueOnce(
                mockPlayer as unknown as Player,
            );

            emitter.sendOwnPlayer();

            expect(mockSocket.emit).toHaveBeenCalledWith('player:info', {
                player: mockPlayer,
            });
        });
    });
});
