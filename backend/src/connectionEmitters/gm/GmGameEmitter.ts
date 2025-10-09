import { Socket } from "socket.io";
import { gameloop } from "../../services/gameloop.js";
import playerRepository from "../../repositories/playerRepository.js";

export default class GmGameEmitter {
    // Singleton instance / register only one GM game emitter at a time
    private static instance: GmGameEmitter | null = null;

    public static registerSocket = (s: Socket) => {
        GmGameEmitter.instance = new GmGameEmitter(s);
    };

    public static unregisterSocket = () => {
        GmGameEmitter.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
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
