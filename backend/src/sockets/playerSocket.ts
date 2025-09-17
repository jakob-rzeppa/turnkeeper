import { Server, Socket } from "socket.io";
import { authenticatePlayer, disconnectPlayer } from "../auth/playerAuth.js";
import playerRepository from "../repositories/playerRepository.js";
import { registerPlayerPlayersHandler } from "../connectionHandlers/playerPlayersHandler.js";

const onPlayerConnection = (socket: Socket): void => {
    const playerId = playerRepository.getPlayerIdByName(
        socket.handshake.auth.playerName
    );

    if (!playerId) {
        console.error("Player ID not found after authentication");
        socket.disconnect(true);
        return;
    }

    console.log(`Player with ID ${playerId} connected: ${socket.id}`);

    registerPlayerPlayersHandler({ socket, playerId });

    socket.on("disconnect", () => {
        disconnectPlayer({ playerId: playerId });
        console.log(`Player with ID ${playerId} disconnected: ${socket.id}`);
    });
};

export const createPlayerSocket = (io: Server): void => {
    const namespace = io.of("/player");

    namespace.use((socket, next) => {
        const { playerName, playerSecret } = socket.handshake.auth;

        const playerId = playerRepository.getPlayerIdByName(playerName);

        if (!playerId) {
            return next(new Error("Player with that name not found"));
        }

        try {
            authenticatePlayer({ playerId, playerSecret });
            next();
        } catch (error: unknown) {
            if (error instanceof Error) {
                next(error);
            }
        }
    });

    namespace.on("connection", onPlayerConnection);
};
