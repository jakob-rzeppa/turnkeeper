import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";

export default class UserGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        this.socket.emit("game", {
            isInitialized: gameloop.isInitialized(),
            round: gameloop.getRoundInformation(),
        });
    }
}
