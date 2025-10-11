import { Server, Socket } from "socket.io";

import { isUserSecretValid } from "../auth/userAuth.js";
import UserController from "../connectionControllers/UserController.js";
import playerRepository from "../repositories/playerRepository.js";
import logger from "../services/logger.js";

const onUserConnection = (socket: Socket): void => {
    const playerId = playerRepository.getPlayerIdByName(
        socket.handshake.auth.playerName
    );

    if (!playerId) {
        logger.error({
            details: { playerId },
            message: "User connection failed: Player not found",
        });
        socket.disconnect();
        return;
    }

    if (!isUserSecretValid(playerId, socket.handshake.auth.playerSecret)) {
        logger.error({
            details: { playerId },
            message: "User connection failed: Invalid secret",
        });
        socket.disconnect();
        return;
    }

    if (UserController.isConnected(playerId)) {
        logger.error({
            details: { playerId },
            message:
                "User connection failed: User for Player already connected",
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
