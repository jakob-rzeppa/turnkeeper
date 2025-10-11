import { Socket } from "socket.io";

import playerRepository from "../../repositories/playerRepository.js";
import { gameloop } from "../../services/gameloop.js";

export default class GmGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        const playerOrder = gameloop.getPlayerOrder();
        const playerOrderWithNames = playerOrder.map((id, index) => ({
            id,
            name:
                playerRepository.getPlayerNameById(id) ??
                `Player ${(index + 1).toString()}`,
        }));

        this.socket.emit("game:turn", {
            isInitialized: gameloop.isInitialized(),
            playerOrder: playerOrderWithNames,
            round: gameloop.getRoundInformation(),
        });
    }
}
