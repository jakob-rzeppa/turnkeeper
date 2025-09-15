import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";
import gmController from "./controllers/gmController.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
    cors: {
        origin: "*",
    },
});

io.on("connection", (socket) => {
    const gmConnection = socket.handshake.auth.gmConnection;

    if (gmConnection) {
        gmController.initConnection(socket);
        gmController.sendPlayerData();
    }
});

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
