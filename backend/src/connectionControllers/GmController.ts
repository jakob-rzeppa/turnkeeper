import { Socket } from "socket.io";
import GmGameListener from "../connectionListeners/gm/GmGameListener.js";
import GmPlayersListener from "../connectionListeners/gm/GmPlayersListener.js";
import GmGameEmitter from "../connectionEmitters/gm/GmGameEmitter.js";
import GmLogsEmitter from "../connectionEmitters/gm/GmLogsEmitter.js";
import GmPlayersEmitter from "../connectionEmitters/gm/GmPlayersEmitter.js";

export default class GmController {
    // Singleton instance / register only one GM players listener at a time
    private static instance: GmController | null = null;

    public static isConnected = (): boolean => {
        return this.instance !== null;
    };

    public static registerSocket = (s: Socket) => {
        GmController.instance = new GmController(s);
    };

    public static unregisterSocket = () => {
        GmController.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    // Listeners
    public gmGameListener: GmGameListener;
    public gmPlayersListener: GmPlayersListener;

    // Emitters
    public gmGameEmitter: GmGameEmitter;
    public gmLogsEmitter: GmLogsEmitter;
    public gmPlayersEmitter: GmPlayersEmitter;

    private constructor(s: Socket) {
        // Initialize listeners
        this.gmGameListener = new GmGameListener(s);
        this.gmPlayersListener = new GmPlayersListener(s);

        // Initial emits
        this.gmGameEmitter = new GmGameEmitter(s);
        this.gmPlayersEmitter = new GmPlayersEmitter(s);
        this.gmLogsEmitter = new GmLogsEmitter(s);
    }
}
