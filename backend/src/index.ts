import { createServer } from "http";
import { Server, Socket } from "socket.io";

import config from "./config/config.js";

import { registerGmPlayersHandler } from "./connectionHandlers/gmPlayersHandler.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
    cors: {
        origin: "*",
    },
});

const onGmConnection = (socket: Socket) => {
    console.log(`GM connected: ${socket.id}`);

    registerGmPlayersHandler(socket);

    socket.on("disconnect", () => {
        console.log(`GM disconnected: ${socket.id}`);
    });
};

io.of("/gm").on("connection", onGmConnection);

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
