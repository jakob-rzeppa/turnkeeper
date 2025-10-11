import { Socket } from "socket.io";

import { LogEntry } from "../../types/logTypes.js";

export default class GmLogsEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;
    }

    public sendLog = (logEntry: LogEntry) => {
        this.socket.emit("log", logEntry);
    };
}
