import { Socket } from 'socket.io';

import GmGameEmitter from '../connectionEmitters/gm/GmGameEmitter.js';
import GmLogsEmitter from '../connectionEmitters/gm/GmLogsEmitter.js';
import GmMessagesEmitter from '../connectionEmitters/gm/GmMessagesEmitter.js';
import GmPlayersEmitter from '../connectionEmitters/gm/GmPlayersEmitter.js';
import GmGameListener from '../connectionListeners/gm/GmGameListener.js';
import GmPlayersListener from '../connectionListeners/gm/GmPlayersListener.js';

export default class GmController {
    // Singleton instance / register only one GM players listener at a time
    private static instance: GmController | null = null;

    // Emitters
    public gmGameEmitter: GmGameEmitter;
    public gmLogsEmitter: GmLogsEmitter;
    public gmPlayersEmitter: GmPlayersEmitter;
    public gmMessagesEmitter: GmMessagesEmitter;

    // Listeners
    public gmGameListener: GmGameListener;
    public gmPlayersListener: GmPlayersListener;

    private constructor(s: Socket) {
        // Initialize emitters
        this.gmGameEmitter = new GmGameEmitter(s);
        this.gmPlayersEmitter = new GmPlayersEmitter(s);
        this.gmLogsEmitter = new GmLogsEmitter(s);
        this.gmMessagesEmitter = new GmMessagesEmitter(s);

        // Initialize listeners
        this.gmGameListener = new GmGameListener(s);
        this.gmPlayersListener = new GmPlayersListener(s);
    }

    public static getInstance = (): GmController | null => {
        return this.instance;
    };
    public static isConnected = (): boolean => {
        return this.instance !== null;
    };
    public static registerSocket = (s: Socket): void => {
        GmController.instance = new GmController(s);
    };

    public static unregisterSocket = (): void => {
        GmController.instance = null;
    };
}
