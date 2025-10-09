import { Server, Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";
import logger from "../services/logger.js";
import UserController from "../connectionControllers/UserController.js";
import { isUserSecretValid } from "../auth/userAuth.js";

const onUserConnection = (socket: Socket): void => {
    const playerId = playerRepository.getPlayerIdByName(
        socket.handshake.auth.playerName
    );

    if (!playerId) {
        logger.error({
            message: "User connection failed: Player not found",
            details: { playerId },
        });
        socket.disconnect();
        return;
    }

    if (!isUserSecretValid(playerId, socket.handshake.auth.playerSecret)) {
        logger.error({
            message: "User connection failed: Invalid secret",
            details: { playerId },
        });
        socket.disconnect();
        return;
    }

    if (UserController.isConnected(playerId)) {
        logger.error({
            message:
                "User connection failed: User for Player already connected",
            details: { playerId },
        });
        socket.disconnect();
        return;
    }

    logger.info({
        message: "User connected",
        details: { playerId },
    });

    UserController.registerSocket(playerId, socket);

    socket.on("disconnect", () => {
        UserController.unregisterSocket(playerId);
        logger.info({
            message: "User disconnected",
            details: { playerId },
        });
    });
};

export const createUserSocket = (io: Server): void => {
    const namespace = io.of("/user");

    namespace.on("connection", onUserConnection);
};
