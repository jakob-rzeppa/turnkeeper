import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

/*
 * The gm/playersHandler is responsible for handling the player-specific events and data for the GM.
 * For example, creating new players and sending the list of all players.
 */

const sendPlayers = (socket: Socket) => {
    const players = playerRepository.getAllPlayers();
    socket.emit("players", players);
};

const createPlayer = (playerData: { name: string }) => {
    playerRepository.createPlayer(playerData.name);
};

export const registerPlayersHandler = (socket: Socket) => {
    sendPlayers(socket);

    socket.on("players:create", (playerData) => {
        createPlayer(playerData);
        sendPlayers(socket);
    });
};
