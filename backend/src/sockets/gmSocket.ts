import { Server, Socket } from "socket.io";
import { authenticateGm, disconnectGm } from "../auth/gmAuth.js";
import logger from "../services/logger.js";
import GmLogsHandler from "../connectionHandlers/gm/gmLogsHandler.js";
import GmGameHandler from "../connectionHandlers/gm/gmGameHandler.js";
import GmPlayersHandler from "../connectionHandlers/gm/gmPlayersHandler.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        message: "GM connected",
        data: { socketId: socket.id },
    });

    GmPlayersHandler.registerSocket(socket);
    GmGameHandler.registerSocket(socket);
    GmLogsHandler.registerSocket(socket);

    socket.on("disconnect", () => {
        disconnectGm();
        GmLogsHandler.unregisterSocket();
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
