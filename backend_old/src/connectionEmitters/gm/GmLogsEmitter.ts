import { BackendToGmEventPayloads, LogEntry } from 'shared-types';
import { Socket } from 'socket.io';

export default class GmLogsEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;
    }

    public sendLog = (logEntry: LogEntry) => {
        const payload: BackendToGmEventPayloads['log:entry'] = {
            entry: logEntry,
        };

        this.socket.emit('log:entry', payload);
    };
}
