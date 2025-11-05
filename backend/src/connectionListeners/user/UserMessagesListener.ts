import { UserToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';
import messagesHandler from '../../services/messagesHandler.js';

export default class UserMessagesListener {
    private playerId: number;
    private socket: Socket;

    public constructor(playerId: number, s: Socket) {
        this.playerId = playerId;
        this.socket = s;

        this.socket.on(
            'messages:send',
            ({ content }: UserToBackendEventPayloads['messages:send']) => {
                this.sendMessageToGm(content);
            },
        );
    }

    private sendMessageToGm(content: string) {
        messagesHandler.sendMessageFromPlayer(this.playerId, content);
    }
}
