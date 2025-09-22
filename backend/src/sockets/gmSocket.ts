import { Server, Socket } from "socket.io";
import { registerGmPlayersHandler } from "../connectionHandlers/gm/gmPlayersHandler.js";
import { authenticateGm, disconnectGm } from "../auth/gmAuth.js";
import { registerGmGameHandler } from "../connectionHandlers/gm/gmGameHandler.js";
import logger from "../services/logger.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        message: "GM connected",
        data: { socketId: socket.id },
    });

    registerGmPlayersHandler(socket);
    registerGmGameHandler(socket);

    socket.on("disconnect", () => {
        disconnectGm();
        logger.info({
            message: "GM disconnected",
            data: { socketId: socket.id },
        });
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
