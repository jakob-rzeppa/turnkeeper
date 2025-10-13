import { Server, Socket } from "socket.io";

import { authenticateUser } from "../auth/userAuth.js";
import UserController from "../connectionControllers/UserController.js";
import logger from "../services/logger.js";
import { extractUserCredentials } from "../util/extractUserCredentials.js";

export const handleDisconnect = (playerId: string): void => {
    UserController.unregisterSocket(playerId);
    logger.info({
        details: { playerId },
        message: "User disconnected",
    });
};

export const onUserConnection = (socket: Socket): void => {
    const credentials = extractUserCredentials(socket);

    if (!credentials) {
        logger.error({
            details: { credentials },
            message: "A user tried to connect but player was not found",
        });

        socket.emit("connection_error", {
            code: "INVALID_CREDENTIALS",
            message: "Connection refused: Credentials do not match any player",
        });

        socket.disconnect();
        return;
    }

    if (
        !authenticateUser(
            socket,
            credentials.playerId,
            credentials.playerSecret
        )
    )
        return;

    logger.info({
        details: { playerId: credentials.playerId },
        message: "User connected",
    });

    UserController.registerSocket(credentials.playerId, socket);

    socket.on("disconnect", () => handleDisconnect(credentials.playerId));
};

export const createUserSocket = (io: Server): void => {
    const namespace = io.of("/user");

    namespace.on("connection", onUserConnection);
};
