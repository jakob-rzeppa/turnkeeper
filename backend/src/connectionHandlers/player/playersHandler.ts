import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

const sendPlayerInfo = ({
    socket,
    playerId,
}: {
    socket: Socket;
    playerId: string;
}) => {
    const players = playerRepository.getPlayerById(playerId);

    if (!players) {
        console.error(`Player with ID ${playerId} not found`);
        socket.disconnect(true);
        return;
    }

    const { id, name, stats } = players;

    socket.emit("players", { id, name, stats });
};

export const registerPlayersHandler = ({
    socket,
    playerId,
}: {
    socket: Socket;
    playerId: string;
}) => {
    socket.on("getPlayers", () => {
        sendPlayerInfo({ socket, playerId });
    });

    // Send initial data
    sendPlayerInfo({ socket, playerId });
};
