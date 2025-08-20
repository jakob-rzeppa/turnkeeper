import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";
import authenticateGm from "./auth/gmAuth.js";
import { authenticatePlayer } from "./auth/playerAuth.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
    cors: {
        origin: "*",
    },
});

io.on("connection", (socket) => {
    const gmConnection = socket.handshake.auth.gmConnection;

    if (gmConnection && !authenticateGm(socket)) {
        socket.disconnect();
        return;
    }

    if (!gmConnection && !authenticatePlayer(socket)) {
        socket.disconnect();
        return;
    }

    socket.on("message", (message) => {
        console.log(`Received message: ${message}`);
    });

    // socket.on("disconnect", () => {
    //     console.log(`Client disconnected: ${socket.id}`);
    // });
});

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
