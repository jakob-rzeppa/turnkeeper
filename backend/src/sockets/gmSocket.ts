import { Server, Socket } from "socket.io";
import { registerGmPlayersHandler } from "../connectionHandlers/gm/gmPlayersHandler.js";
import { authenticateGm, disconnectGm } from "../auth/gmAuth.js";

const onGmConnection = (socket: Socket) => {
    console.log(`GM connected: ${socket.id}`);

    registerGmPlayersHandler(socket);

    socket.on("disconnect", () => {
        disconnectGm();
        console.log(`GM disconnected: ${socket.id}`);
    });
};

export const createGmSocket = (io: Server) => {
    const namespace = io.of("/gm");

    namespace.use((_, next) => {
        try {
            authenticateGm();
            next();
        } catch (error: unknown) {
            if (error instanceof Error) {
                next(error);
            }
        }
    });

    namespace.on("connection", onGmConnection);
};
