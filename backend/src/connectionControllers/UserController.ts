import { Socket } from "socket.io";
import UserPlayersEmitter from "../connectionEmitters/user/UserPlayersEmitter.js";

export default class UserController {
    // Multiple instances / register one user controller per playerId
    private static instances: Map<string, UserController> = new Map();

    public static registerSocket = (playerId: string, s: Socket) => {
        if (!this.instances.has(playerId)) {
            this.instances.set(playerId, new UserController(playerId, s));
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

    // Emitters
    public userPlayersEmitter: UserPlayersEmitter;

    private constructor(playerId: string, s: Socket) {
        this.userPlayersEmitter = new UserPlayersEmitter(playerId, s);
    }
}
