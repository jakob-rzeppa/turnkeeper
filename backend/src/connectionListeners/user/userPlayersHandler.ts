import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

const sendPlayerInfo = ({
    socket,
    playerId,
}: {
    socket: Socket;
    playerId: string;
}) => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player) {
        console.error(`Player with ID ${playerId} not found`);
        socket.disconnect(true);
        return;
    }

    const { id, name, stats } = player;

    socket.emit("player", { id, name, stats });
};

export const registerUserPlayersHandler = ({
    socket,
    playerId,
}: {
    socket: Socket;
    playerId: string;
}) => {
    // Send initial data
    sendPlayerInfo({ socket, playerId });
};
