import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

export default class UserPlayersEmitter {
    // Multiple instances / register one user players emitter per connected player
    private static instances: Map<string, UserPlayersEmitter> = new Map();

    public static registerSocket = (playerId: string, s: Socket) => {
        if (!this.instances.has(playerId)) {
            this.instances.set(playerId, new UserPlayersEmitter(playerId, s));
        }
    };

    public static unregisterSocket = (playerId: string) => {
        this.instances.delete(playerId);
    };

    public static getInstance = (playerId: string) => {
        return this.instances.get(playerId);
    };

    public static isConnected = (playerId: string): boolean => {
        return this.instances.has(playerId);
    };

    private socket: Socket;
    private playerId: string;

    private constructor(playerId: string, s: Socket) {
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
