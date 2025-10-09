import { Server, Socket } from "socket.io";
import logger from "../services/logger.js";
import GmController from "../connectionControllers/GmController.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        message: "GM connected",
        details: { socketId: socket.id },
    });

    if (GmController.isConnected()) {
        logger.error({
            message: "GM connection failed: GM already connected",
            details: { socketId: socket.id },
        });
        socket.disconnect();
        return;
    }

    GmController.registerSocket(socket);

    socket.on("disconnect", () => {
        GmController.unregisterSocket();
        logger.info({
            message: "GM disconnected",
            details: { socketId: socket.id },
        });
    });
};

export const createGmSocket = (io: Server) => {
    const namespace = io.of("/gm");

    namespace.on("connection", onGmConnection);
};
