import { Server, Socket } from "socket.io";
import { authenticatePlayer, disconnectPlayer } from "../auth/playerAuth.js";

const onPlayerConnection = (socket: Socket): void => {
    console.log(`Player connected: ${socket.id}`);

    socket.on("disconnect", () => {
        disconnectPlayer({ playerId: socket.id });
        console.log(`Player disconnected: ${socket.id}`);
    });
};

export const createPlayerSocket = (io: Server): void => {
    const namespace = io.of("/player");

    namespace.use((socket, next) => {
        const { playerName, playerSecret } = socket.handshake.auth;

        try {
            authenticatePlayer({ playerName, playerSecret });
            next();
        } catch (error: unknown) {
            if (error instanceof Error) {
                next(error);
            }
        }
    });

    namespace.on("connection", onPlayerConnection);
};
