import { Server, Socket } from "socket.io";

import { isUserSecretValid } from "../auth/userAuth.js";
import UserController from "../connectionControllers/UserController.js";
import playerRepository from "../repositories/playerRepository.js";
import logger from "../services/logger.js";

const onUserConnection = (socket: Socket): void => {
    if (
        !socket.handshake.auth.playerName ||
        !socket.handshake.auth.playerSecret ||
        !(typeof socket.handshake.auth.playerName === "string") ||
        !(typeof socket.handshake.auth.playerSecret === "string")
    ) {
        logger.error({
            details: { handshakeAuth: socket.handshake.auth },
            message: "A user tried to connect without proper credentials",
        });

        socket.emit("connection_error", {
            code: "MISSING_CREDENTIALS",
            message: "Connection refused: Missing player name or secret",
        });

        socket.disconnect();
        return;
    }

    const playerId = playerRepository.getPlayerIdByName(
        socket.handshake.auth.playerName
    );

    if (!playerId) {
        logger.error({
            details: { playerId },
            message: "A user tried to connect but player was not found",
        });

        socket.emit("connection_error", {
            code: "PLAYER_NOT_FOUND",
            message: "Connection refused: Player not found",
        });

        socket.disconnect();
        return;
    }

    if (!isUserSecretValid(playerId, socket.handshake.auth.playerSecret)) {
        logger.error({
            details: { playerId },
            message: "A user tried to connect but provided an invalid secret",
        });

        socket.emit("connection_error", {
            code: "INVALID_SECRET",
            message: "Connection refused: Invalid player secret",
        });

        socket.disconnect();
        return;
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
        return;
    }

    logger.info({
        details: { playerId },
        message: "User connected",
    });

    UserController.registerSocket(playerId, socket);

    socket.on("disconnect", () => {
        UserController.unregisterSocket(playerId);
        logger.info({
            details: { playerId },
            message: "User disconnected",
        });
    });
};

export const createUserSocket = (io: Server): void => {
    const namespace = io.of("/user");

    namespace.on("connection", onUserConnection);
};
