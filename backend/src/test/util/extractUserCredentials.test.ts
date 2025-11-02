import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, Mock, vi } from 'vitest';

import playerRepository from '../../repositories/playerRepository.js';
import { extractUserCredentials } from '../../util/extractUserCredentials.js';

vi.mock('../../repositories/playerRepository', () => ({
    default: {
        getPlayerIdByName: vi.fn(),
    },
}));

describe('extractUserCredentials', () => {
    let socket: Socket;

    beforeEach(() => {
        socket = {
            handshake: {
                auth: {},
            },
        } as unknown as Socket;
    });

    describe('when playerName or playerSecret is missing', () => {
        it('returns null if playerName is missing', () => {
            socket.handshake.auth = { playerSecret: 'secret' };
            const result = extractUserCredentials(socket);
            expect(result).toBeNull();
        });

        it('returns null if playerSecret is missing', () => {
            socket.handshake.auth = { playerName: 'name' };
            const result = extractUserCredentials(socket);
            expect(result).toBeNull();
        });
    });

    describe('when playerName or playerSecret is not a string', () => {
        it('returns null if playerName is not a string', () => {
            socket.handshake.auth = { playerName: 123, playerSecret: 'secret' };
            const result = extractUserCredentials(socket);
            expect(result).toBeNull();
        });

        it('returns null if playerSecret is not a string', () => {
            socket.handshake.auth = { playerName: 'name', playerSecret: 456 };
            const result = extractUserCredentials(socket);
            expect(result).toBeNull();
        });
    });

    describe('when playerId cannot be found', () => {
        it('returns null', () => {
            socket.handshake.auth = {
                playerName: 'name',
                playerSecret: 'secret',
            };
            (playerRepository.getPlayerIdByName as Mock).mockReturnValue(null);

            const result = extractUserCredentials(socket);
            expect(result).toBeNull();
        });
    });

    describe('when all values are valid', () => {
        it('returns the playerId and playerSecret', () => {
            socket.handshake.auth = {
                playerName: 'name',
                playerSecret: 'secret',
            };
            (playerRepository.getPlayerIdByName as Mock).mockReturnValue('player-id');

            const result = extractUserCredentials(socket);
            expect(result).toEqual({
                playerId: 'player-id',
                playerSecret: 'secret',
            });
        });
    });
});
