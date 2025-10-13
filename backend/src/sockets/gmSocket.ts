import { Server, Socket } from "socket.io";

import GmController from "../connectionControllers/GmController.js";
import logger from "../services/logger.js";

export const handleDisconnect = (socket: Socket) => {
    GmController.unregisterSocket();
    logger.info({
        details: { socketId: socket.id },
        message: "GM disconnected",
    });
};

export const onGmConnection = (socket: Socket) => {
    if (GmController.isConnected()) {
        logger.warn({
            details: { socketId: socket.id },
            message:
                "A GM tried to connect, but another GM is already connected",
        });

        // Send error to client before disconnecting
        socket.emit("connection_error", {
            code: "GM_ALREADY_CONNECTED",
            message: "GM connection refused: Another GM is already connected",
        });

        socket.disconnect();
        return;
    }

    GmController.registerSocket(socket);

    logger.info({
        details: { socketId: socket.id },
        message: "GM connected",
    });

    socket.on("disconnect", () => { handleDisconnect(socket); });
};

export const createGmSocket = (io: Server) => {
    const namespace = io.of("/gm");

    namespace.on("connection", onGmConnection);
};
