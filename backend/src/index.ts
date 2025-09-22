import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";

import { createGmSocket } from "./sockets/gmSocket.js";
import { createUserSocket } from "./sockets/userSocket.js";
import logger from "./services/logger.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
    cors: {
        origin: "*",
    },
});

createGmSocket(io);
createUserSocket(io);

httpServer.listen(port, () => {
    `Server is running on port ${port}`;

    logger.info({
        message: "Server is running",
        data: { port },
    });
});
