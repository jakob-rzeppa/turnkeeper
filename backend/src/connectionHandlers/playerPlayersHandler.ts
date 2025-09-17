import { Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";

const sendPlayerInfo = ({
    socket,
    playerId,
}: {
    socket: Socket;
    playerId: string;
}) => {
    const players = playerRepository.getPlayerById(playerId);

    if (!players) {
        throw new Error("Player not found");
    }

    const { id, name, stats } = players;

    socket.emit("players", { id, name, stats });
};

export const registerPlayerPlayersHandler = ({
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
