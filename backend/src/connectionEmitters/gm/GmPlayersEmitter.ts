import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

export default class GmPlayersEmitter {
    // Singleton instance / register only one GM players emitter at a time
    private static instance: GmPlayersEmitter | null = null;

    public static registerSocket = (s: Socket) => {
        GmPlayersEmitter.instance = new GmPlayersEmitter(s);
    };

    public static unregisterSocket = () => {
        GmPlayersEmitter.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendPlayers();
    }

    public sendPlayers() {
        const players = playerRepository.getAllPlayers();
        this.socket.emit("players", players);
    }
}
