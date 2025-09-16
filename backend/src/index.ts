import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";

import { createGmSocket } from "./sockets/gmSocket.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
    cors: {
        origin: "*",
    },
});

createGmSocket(io);

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
