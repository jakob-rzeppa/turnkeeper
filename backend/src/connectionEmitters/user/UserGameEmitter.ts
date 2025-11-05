import { BackendToUserEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import gameStateHandler from '../../services/gameStateHandler.js';

export default class UserGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        const gameState = gameStateHandler.getGameState();

        const payload: BackendToUserEventPayloads['game:info'] = {
            gameState: gameState
                ? {
                      currentPlayerIndex: gameState.currentPlayerIndex,
                      id: gameState.id,
                      notes: gameState.notes,
                      playerOrder: gameState.playerOrder,
                      roundNumber: gameState.roundNumber,
                  }
                : null,
        };

        this.socket.emit('game:info', payload);
    }
}
