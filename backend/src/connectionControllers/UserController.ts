import { Socket } from "socket.io";

import UserGameEmitter from "../connectionEmitters/user/UserGameEmitter.js";
import UserPlayersEmitter from "../connectionEmitters/user/UserPlayersEmitter.js";

export default class UserController {
    // Multiple instances / register one user controller per playerId
    private static instances = new Map<string, UserController>();

    public socket: Socket;

    public userGameEmitter: UserGameEmitter;
    // Emitters
    public userPlayersEmitter: UserPlayersEmitter;

    private constructor(playerId: string, s: Socket) {
        this.socket = s;

        this.userPlayersEmitter = new UserPlayersEmitter(playerId, this.socket);
        this.userGameEmitter = new UserGameEmitter(this.socket);
    }

    public static forEachInstance = (
        cb: (userController: UserController) => void
    ) => {
        this.instances.forEach((userController) => { cb(userController); });
    };

    public static getAllInstances = () => {
        return Array.from(this.instances.values());
    };

    public static getInstance = (playerId: string) => {
        return this.instances.get(playerId);
    };

    public static isConnected = (playerId: string): boolean => {
        return this.instances.has(playerId);
    };

    public static registerSocket = (playerId: string, s: Socket) => {
        if (!this.instances.has(playerId)) {
            this.instances.set(playerId, new UserController(playerId, s));
        }
    };

    public static unregisterSocket = (playerId: string) => {
        this.instances.delete(playerId);
    };

    public disconnect() {
        this.socket.disconnect();
    }
}
