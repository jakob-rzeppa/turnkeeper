import { Server, Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";
import logger from "../services/logger.js";
import UserPlayersEmitter from "../connectionEmitters/user/UserPlayersEmitter.js";

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

    if (UserPlayersEmitter.isConnected(playerId)) {
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

    UserPlayersEmitter.registerSocket(playerId, socket);

    socket.on("disconnect", () => {
        UserPlayersEmitter.unregisterSocket(playerId);
        logger.info({
            message: "User disconnected",
            details: { playerId, socketId: socket.id },
        });
    });
};

export const createUserSocket = (io: Server): void => {
    const namespace = io.of("/user");

    namespace.on("connection", onUserConnection);
};
