import { Server, Socket } from "socket.io";
import { authenticateGm, disconnectGm } from "../auth/gmAuth.js";
import logger from "../services/logger.js";
import GmController from "../connectionControllers/GmController.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        message: "GM connected",
        details: { socketId: socket.id },
    });

    GmController.registerSocket(socket);

    socket.on("disconnect", () => {
        disconnectGm();
        GmController.unregisterSocket();
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
