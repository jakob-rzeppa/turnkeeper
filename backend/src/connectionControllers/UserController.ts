import { Socket } from 'socket.io';

import UserGameEmitter from '../connectionEmitters/user/UserGameEmitter.js';
import UserMessagesEmitter from '../connectionEmitters/user/UserMessagesEmitter.js';
import UserPlayersEmitter from '../connectionEmitters/user/UserPlayersEmitter.js';

export default class UserController {
    // Multiple instances / register one user controller per playerId
    private static instances = new Map<number, UserController>();

    public playerId: number;
    public socket: Socket;

    // Emitters
    public userGameEmitter: UserGameEmitter;
    public userPlayersEmitter: UserPlayersEmitter;
    public userMessagesEmitter: UserMessagesEmitter;

    private constructor(playerId: number, s: Socket) {
        this.playerId = playerId;
        this.socket = s;

        this.userPlayersEmitter = new UserPlayersEmitter(this.playerId, this.socket);
        this.userGameEmitter = new UserGameEmitter(this.socket);
        this.userMessagesEmitter = new UserMessagesEmitter(this.playerId, this.socket);
    }

    public static getAllInstances = (): UserController[] => {
        return Array.from(this.instances.values());
    };

    public static getInstance = (playerId: number): undefined | UserController => {
        return this.instances.get(playerId);
    };

    public static isConnected = (playerId: number): boolean => {
        return this.instances.has(playerId);
    };

    public static registerSocket = (playerId: number, s: Socket): void => {
        if (!this.instances.has(playerId)) {
            this.instances.set(playerId, new UserController(playerId, s));
        }
    };

    public static unregisterSocket = (playerId: number): void => {
        this.instances.delete(playerId);
    };

    public disconnect(): void {
        this.socket.disconnect();
    }
}
