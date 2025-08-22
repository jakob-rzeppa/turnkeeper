import { Socket } from "socket.io";
import statsService from "../services/statsService.js";

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

        gmController.sendStats();
    },
    sendStats() {
        if (!connection.socket) {
            console.error("No Game Master connection established.");
            return;
        }

        console.log("Sending stats to Game Master...");

        const stats = statsService.getStatsForAllPlayers();

        connection.socket.emit("stats", stats);
    },
};

export default gmController;
