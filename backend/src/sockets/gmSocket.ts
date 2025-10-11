import { Server, Socket } from "socket.io";

import GmController from "../connectionControllers/GmController.js";
import logger from "../services/logger.js";

const onGmConnection = (socket: Socket) => {
    logger.info({
        details: { socketId: socket.id },
        message: "GM connected",
    });

    if (GmController.isConnected()) {
        logger.error({
            details: { socketId: socket.id },
            message: "GM connection failed: GM already connected",
        });
        socket.disconnect();
        return;
    }

    GmController.registerSocket(socket);

    socket.on("disconnect", () => {
        GmController.unregisterSocket();
        logger.info({
            details: { socketId: socket.id },
            message: "GM disconnected",
        });
    });
};

export const createGmSocket = (io: Server) => {
    const namespace = io.of("/gm");

    namespace.on("connection", onGmConnection);
};
