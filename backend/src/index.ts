import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";

import { createGmSocket } from "./sockets/gmSocket.js";
import { createUserSocket } from "./sockets/userSocket.js";

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
    console.log(`Server is running on port ${port}`);
});
