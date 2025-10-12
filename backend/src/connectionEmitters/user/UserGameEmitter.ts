import { BackendToUserPayloads } from "shared-types";
import { Socket } from "socket.io";

import playerRepository from "../../repositories/playerRepository.js";
import { gameloop } from "../../services/gameloop.js";

export default class UserGameEmitter {
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

        const payload: BackendToUserPayloads["game:info"] = {
            isInitialized: gameloop.isInitialized(),
            playerOrder: playerOrderWithNames,
            round: gameloop.getRoundInformation(),
        };

        this.socket.emit("game:info", payload);
    }
}
