import { Socket } from 'socket.io';

import UserController from '../connectionControllers/UserController.js';
import playerRepository from '../repositories/playerRepository.js';
import logger from '../services/logger.js';
/**
 * The secret is a simple string that is generated when the player is created.
 * It is not meant to be secure, just to prevent accidental impersonation.
 *
 * @returns true if authentication was successful, false otherwise
 */
export const authenticateUser = (
    socket: Socket,
    playerId: number,
    playerSecret: string,
): boolean => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player || playerSecret !== player.secret) {
        logger.error({
            details: { playerId },
            message: 'A user tried to connect but provided an invalid secret',
        });

        socket.emit('connection_error', {
            code: 'INVALID_SECRET',
            message: 'Connection refused: Invalid player secret',
        });

        socket.disconnect();
        return false;
    }

    if (UserController.isConnected(playerId)) {
        logger.error({
            details: { playerId },
            message:
                'A user tried to connect but another user is already connected for this player',
        });

        socket.emit('connection_error', {
            code: 'PLAYER_ALREADY_CONNECTED',
            message: 'Connection refused: This player is already connected',
        });

        socket.disconnect();
        return false;
    }

    return true;
};
