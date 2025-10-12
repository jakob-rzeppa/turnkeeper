import { BackendToGmEventPayloads } from "shared-types";
import { Socket } from "socket.io";

import playerRepository from "../../repositories/playerRepository.js";

export default class GmPlayersEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendPlayers();
    }

    public sendPlayers() {
        const players = playerRepository.getAllPlayers();

        const payload: BackendToGmEventPayloads["players:info"] = players;

        this.socket.emit("players:info", payload);
    }
}
