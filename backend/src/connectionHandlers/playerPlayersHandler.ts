import { Socket } from "socket.io";
import playerRepository from "../repositories/playerRepository.js";

const sendPlayerInfo = (socket: Socket) => {
    const players = playerRepository.getPlayerById(socket.id);
    socket.emit("players", players);
};
