import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

export default class UserPlayersEmitter {
    private socket: Socket;
    private playerId: string;

    public constructor(playerId: string, s: Socket) {
        this.playerId = playerId;
        this.socket = s;

        // Initial emits
        this.sendOwnPlayer();
    }

    public sendOwnPlayer() {
        const player = playerRepository.getPlayerById(this.playerId);
        this.socket.emit("player", player);
    }
}
