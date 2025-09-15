import { Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";

const sendPlayers = (socket: Socket) => {
    const players = playerRepository.getAllPlayers();
    socket.emit("players", players);
};

const addPlayer = (playerData: { name: string }) => {
    playerRepository.addPlayer(playerData.name);
};

export const registerGmPlayersHandler = (socket: Socket) => {
    sendPlayers(socket);

    socket.on("players:create", (playerData) => {
        addPlayer(playerData);
        sendPlayers(socket);
    });
};
