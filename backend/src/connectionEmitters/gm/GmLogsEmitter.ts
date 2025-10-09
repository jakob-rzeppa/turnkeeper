import { Socket } from "socket.io";
import { LogEntry } from "../../types/logTypes.js";

export default class GmLogsEmitter {
    // Singleton instance / register only one GM logs emitter at a time
    private static instance: GmLogsEmitter | null = null;

    public static registerSocket = (s: Socket) => {
        GmLogsEmitter.instance = new GmLogsEmitter(s);
    };

    public static unregisterSocket = () => {
        GmLogsEmitter.instance = null;
    };

    public static getInstance = () => {
        return this.instance;
    };

    private socket: Socket;

    private constructor(s: Socket) {
        this.socket = s;
    }

    public sendLog = (logEntry: LogEntry) => {
        this.socket.emit("log", logEntry);
    };
}
