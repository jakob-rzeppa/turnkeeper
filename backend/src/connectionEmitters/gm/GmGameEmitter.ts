import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";
import playerRepository from "../../repositories/playerRepository.js";

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
                playerRepository.getPlayerNameById(id) || `Player ${index + 1}`,
        }));

        this.socket.emit("game:turn", {
            playerOrder: playerOrderWithNames,
            isInitialized: gameloop.isInitialized(),
            round: gameloop.getRoundInformation(),
        });
    }
}
