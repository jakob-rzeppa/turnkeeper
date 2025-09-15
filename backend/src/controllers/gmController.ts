import { Socket } from "socket.io";
import playerService from "../repositories/playerRepository.js";

export const connection: { socket: Socket | null } = { socket: null };

export const initConnection = (socket: Socket) => {
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
};

export const sendPlayerData = () => {
    if (connection.socket) {
        const players = playerService.getAllPlayers();

        connection.socket.emit("players", players);

        console.log(`Sent player data to Game Master`);
    } else {
        console.warn("No Game Master connected. Cannot send player data.");
    }
};
