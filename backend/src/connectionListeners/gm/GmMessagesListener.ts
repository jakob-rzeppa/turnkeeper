import { GmToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import messagesHandler from '../../services/messagesHandler.js';

/**
 * Listener for GM actions to control the game loop (turns, rounds, player order)
 */

export default class GmMessagesListener {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        this.socket.on(
            'messages:send',
            ({ playerId, content }: GmToBackendEventPayloads['messages:send']) => {
                this.sendMessage(playerId, content);
            },
        );
    }

    private sendMessage(playerId: number, content: string) {
        messagesHandler.sendMessageToPlayer(playerId, content);
    }
}
