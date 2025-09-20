import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";
import playerRepository from "../../repositories/playerRepository.js";

const sendPlayerOrder = (socket: Socket) => {
    const playerOrder = gameloop.getPlayerOrder();

    const playerOrderWithNames = playerOrder.map((id, index) => ({
        id,
        name: playerRepository.getPlayerNameById(id) || `Player ${index + 1}`,
    }));
    socket.emit("gameloop:order", { playerOrder: playerOrderWithNames });
};

const initGameLoop = () => {
    const allPlayers = playerRepository.getAllPlayers();
    const playerIds = allPlayers.map((p) => p.id);
    gameloop.init(playerIds);
};

const nextTurn = () => {
    gameloop.nextTurn();
};

export const registerGmTurnHandler = (socket: Socket) => {
    sendPlayerOrder(socket);

    socket.on("gameloop:next", () => {
        nextTurn();
    });

    socket.on("gameloop:init", () => {
        initGameLoop();
        sendPlayerOrder(socket);
    });
};
