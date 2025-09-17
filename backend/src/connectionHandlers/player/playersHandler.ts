import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

/*
 * The player/playersHandler is responsible for handling the player-specific events and data relevant for each player.
 * For example, sending the player's own data, or public data of other players.
 */

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
