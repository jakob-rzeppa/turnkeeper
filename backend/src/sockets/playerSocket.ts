import { Server, Socket } from "socket.io";

const onPlayerConnection = (socket: Socket): void => {
    console.log(`Player connected: ${socket.id}`);

    socket.on("disconnect", () => {
        console.log(`Player disconnected: ${socket.id}`);
    });
};

export const createPlayerSocket = (io: Server): void => {
    const namespace = io.of("/player");

    namespace.on("connection", onPlayerConnection);
};
