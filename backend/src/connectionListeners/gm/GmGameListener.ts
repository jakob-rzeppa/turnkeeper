import { GmToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import gameStateHandler from '../../services/gameStateHandler.js';

/**
 * Listener for GM actions to control the game loop (turns, rounds, player order)
 */

export default class GmGameListener {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        this.socket.on(
            'game:init',
            ({ playerIdsInOrder }: GmToBackendEventPayloads['game:init']) => {
                this.initGameloop(playerIdsInOrder);
            },
        );

        this.socket.on('game:turn:next', () => {
            this.nextTurn();
        });

        this.socket.on('game:end', () => {
            this.endGame();
        });

        this.socket.on(
            'game:playerOrder:update',
            ({ playerIdsInOrder }: GmToBackendEventPayloads['game:playerOrder:update']) => {
                this.updatePlayerOrder(playerIdsInOrder);
            },
        );
    }

    private endGame() {
        gameStateHandler.deleteGameState();
    }

    private initGameloop(playerIdsInOrder: number[]) {
        gameStateHandler.initGameState(playerIdsInOrder);
    }

    private nextTurn() {
        gameStateHandler.nextTurn();
    }

    private updatePlayerOrder(playerIdsInOrder: number[]) {
        gameStateHandler.updatePlayerOrder(playerIdsInOrder);
    }
}
