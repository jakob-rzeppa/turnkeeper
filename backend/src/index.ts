import { createServer } from "http";
import { Server } from "socket.io";

import config from "./config/config.js";

const port = config.port;

const httpServer = createServer();
const io = new Server(httpServer, {
  cors: {
    origin: "*"
  }
});

const users = new Map<string, { isConnected: boolean }>();

io.on("connection", (socket) => {
  const username = socket.handshake.auth.username;

  if (!username) {
    console.error(`Connection refused to ${socket.id}: Username is required for authentication`);
    socket.disconnect();
  }

  if (users.get(username)?.isConnected) {
    console.error(`Connection refused to ${socket.id}: User ${username} is already connected`);
    socket.disconnect();
    return;
  }

  users.set(username, { isConnected: true });

  console.log(`Client connected: ${socket.id} with username ${username}`);

  socket.on("message", (message) => {
    console.log(`Received message from ${username}: ${message}`);
  });

  socket.on("disconnect", () => {
    console.log(`Client disconnected: ${socket.id}`);
    users.set(username, { isConnected: false });
  });
});

httpServer.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
