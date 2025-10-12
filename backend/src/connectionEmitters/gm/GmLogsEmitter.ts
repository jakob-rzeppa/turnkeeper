import { type LogEntryInterface } from "shared-types";
import { Socket } from "socket.io";

export default class GmLogsEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;
    }

    public sendLog = (logEntry: LogEntryInterface) => {
        this.socket.emit("log", logEntry);
    };
}
