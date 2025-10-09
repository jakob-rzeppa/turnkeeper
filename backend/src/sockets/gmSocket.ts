import { Server, Socket } from "socket.io";
import { authenticateGm, disconnectGm } from "../auth/gmAuth.js";
import logger from "../services/logger.js";
import GmLogsEmitter from "../connectionEmitters/gm/GmLogsEmitter.js";
import GmGameListener from "../connectionListeners/gm/GmGameListener.js";
import GmGameEmitter from "../connectionEmitters/gm/GmGameEmitter.js";
import GmPlayersListener from "../connectionListeners/gm/GmPlayersListener.js";
import GmPlayersEmitter from "../connectionEmitters/gm/GmPlayersEmitter.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        message: "GM connected",
        details: { socketId: socket.id },
    });

    GmGameListener.registerSocket(socket);
    GmPlayersListener.registerSocket(socket);

    GmGameEmitter.registerSocket(socket);
    GmPlayersEmitter.registerSocket(socket);
    GmLogsEmitter.registerSocket(socket);

    socket.on("disconnect", () => {
        disconnectGm();
        GmLogsEmitter.unregisterSocket();
        logger.info({
            message: "GM disconnected",
            details: { socketId: socket.id },
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
