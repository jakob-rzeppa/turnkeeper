import { Socket } from "socket.io";
import { LogEntry } from "../../types/logTypes.js";
import logger from "../../services/logger.js";

export default class GmLogsHandler {
    // Singleton instance / register only one GM logs handler at a time
    private static instance: GmLogsHandler | null = null;

    public static registerGmLogsHandler = (s: Socket) => {
        GmLogsHandler.instance = new GmLogsHandler(s);
    };

    public static unregisterGmLogsHandler = () => {
        GmLogsHandler.instance = null;
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
