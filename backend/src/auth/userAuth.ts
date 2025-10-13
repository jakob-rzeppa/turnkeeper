import { Socket } from "socket.io";

import UserController from "../connectionControllers/UserController.js";
import playerRepository from "../repositories/playerRepository.js";
import logger from "../services/logger.js";
/**
 * The secret is a simple string that is generated when the player is created.
 * It is not meant to be secure, just to prevent accidental impersonation.
 *
 * @returns true if the secret is valid, false otherwise.
 */
export const isUserSecretValid = (
    playerId: string,
    providedSecret: string
): boolean => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player) return false;

    const actualSecret = player.secret;
    return actualSecret == providedSecret;
};

export const authenticateUser = (
    socket: Socket,
    playerId: string,
    playerSecret: string
): boolean => {
    if (!isUserSecretValid(playerId, playerSecret)) {
        logger.error({
            details: { playerId },
            message: "A user tried to connect but provided an invalid secret",
        });

        socket.emit("connection_error", {
            code: "INVALID_SECRET",
            message: "Connection refused: Invalid player secret",
        });

        socket.disconnect();
        return false;
    }

    if (UserController.isConnected(playerId)) {
        logger.error({
            details: { playerId },
            message:
                "A user tried to connect but another user is already connected for this player",
        });

        socket.emit("connection_error", {
            code: "PLAYER_ALREADY_CONNECTED",
            message: "Connection refused: This player is already connected",
        });

        socket.disconnect();
        return false;
    }

    return true;
};
