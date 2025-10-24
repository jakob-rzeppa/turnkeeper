import { BackendToGmEventPayloads } from "shared-types";
import { Socket } from "socket.io";

import gameStateHandler from "../../services/gameStateHandler.js";
import logger from "../../services/logger.js";

export default class GmGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        const gameState = gameStateHandler.getGameState();

        if (!gameState) {
            logger.warn({
                message:
                    "No game state found when attempting to emit game info to GM.",
            });
            return;
        }

        const payload: BackendToGmEventPayloads["game:info"] = { gameState };

        this.socket.emit("game:info", payload);
    }
}
