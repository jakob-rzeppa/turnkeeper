import { BackendToGmEventPayloads } from "shared-types";
import { Socket } from "socket.io";

import gameStateHandler from "../../services/gameStateHandler.js";

export default class GmGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        const gameState = gameStateHandler.getGameState();

        const payload: BackendToGmEventPayloads["game:info"] = { gameState };

        this.socket.emit("game:info", payload);
    }
}
