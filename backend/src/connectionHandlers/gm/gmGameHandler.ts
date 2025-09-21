import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";
import playerRepository from "../../repositories/playerRepository.js";

/*
 * Handlers for GM to control the game loop (turns, rounds, player order)
 */

const sendPlayerOrder = (socket: Socket) => {
    const playerOrder = gameloop.getPlayerOrder();

    const playerOrderWithNames = playerOrder.map((id, index) => ({
        id,
        name: playerRepository.getPlayerNameById(id) || `Player ${index + 1}`,
    }));
    socket.emit("game:turn:players:order", {
        playerOrder: playerOrderWithNames,
    });
};

const initGameloop = (playerIdsInOrder: string[]) => {
    gameloop.init(playerIdsInOrder);
};

const nextTurn = () => {
    gameloop.nextTurn();
};

export const registerGmGameHandler = (socket: Socket) => {
    sendPlayerOrder(socket);

    socket.on("game:turn:next", () => {
        nextTurn();
    });

    socket.on(
        "game:init",
        ({ playerIdsInOrder }: { playerIdsInOrder: string[] }) => {
            initGameloop(playerIdsInOrder);
            sendPlayerOrder(socket);
        }
    );
};
