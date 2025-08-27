import { Socket } from "socket.io";

export const connection: { socket: Socket | null } = { socket: null };

const gmController = {
    initConnection(socket: Socket) {
        if (connection.socket) {
            console.error(
                `Game Master connection refused to ${socket.id}: Game master already connected`
            );
            socket.disconnect();
            return;
        }

        console.log(`Game Master connection established to ${socket.id}`);
        connection.socket = socket;

        socket.on("disconnect", () => {
            // Only handle disconnect for authenticated clients
            // Reason: Ensures only the authenticated Game Master disconnecting triggers cleanup.
            console.log(`Removing connection for Game Master: ${socket.id}`);
            connection.socket = null;
        });
    },
};

export default gmController;
