import { Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";

const sendPlayers = (socket: Socket) => {
    const players = playerRepository.getAllPlayers();
    socket.emit("players", players);
};

const createPlayer = (playerData: { name: string }) => {
    playerRepository.createPlayer(playerData.name);
};

export const registerGmPlayersHandler = (socket: Socket) => {
    sendPlayers(socket);

    socket.on("players:create", (playerData) => {
        createPlayer(playerData);
        sendPlayers(socket);
    });
};
