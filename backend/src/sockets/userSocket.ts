import { Server, Socket } from "socket.io";
import { authenticateUser, disconnectUser } from "../auth/userAuth.js";
import playerRepository from "../repositories/playerRepository.js";
import { registerUserPlayersHandler } from "../connectionListeners/user/userPlayersHandler.js";
import logger from "../services/logger.js";

const onUserConnection = (socket: Socket): void => {
    const playerId = playerRepository.getPlayerIdByName(
        socket.handshake.auth.playerName
    );

    if (!playerId) {
        console.error("Player ID not found after authentication");
        socket.disconnect(true);
        return;
    }

    logger.info({
        message: "User connected",
        details: { playerId, socketId: socket.id },
    });

    registerUserPlayersHandler({ socket, playerId });

    socket.on("disconnect", () => {
        disconnectUser({ playerId: playerId });
        logger.info({
            message: "User disconnected",
            details: { playerId, socketId: socket.id },
        });
    });
};

export const createUserSocket = (io: Server): void => {
    const namespace = io.of("/user");

    namespace.use((socket, next) => {
        const { playerName, playerSecret } = socket.handshake.auth;

        const playerId = playerRepository.getPlayerIdByName(playerName);

        if (!playerId) {
            return next(new Error("Player with that name not found"));
        }

        try {
            authenticateUser({ playerId, playerSecret });
            next();
        } catch (error: unknown) {
            if (error instanceof Error) {
                next(error);
            }
        }
    });

    namespace.on("connection", onUserConnection);
};
