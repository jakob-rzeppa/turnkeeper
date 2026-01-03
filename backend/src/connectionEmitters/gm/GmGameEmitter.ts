import { BackendToGmEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import gameStateHandler from '../../services/gameStateHandler.js';
import playerRepository from '../../repositories/playerRepository.js';

export default class GmGameEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendGameInfo();
    }

    public sendGameInfo() {
        const gameState = gameStateHandler.getGameState();

        if (!gameState) {
            const payload: BackendToGmEventPayloads['game:info'] = {
                gameState: null,
            };

            this.socket.emit('game:info', payload);
            return;
        }

        let playerOrderWithNames: { id: number; name: string }[] = gameState.playerOrder.map(
            (playerId) => {
                const player = playerRepository.getPlayerById(playerId);
                return {
                    id: playerId,
                    name: player ? player.name : 'Unknown',
                };
            },
        );

        const payload: BackendToGmEventPayloads['game:info'] = {
            gameState: {
                currentPlayerIndex: gameState.currentPlayerIndex,
                id: gameState.id,
                notes: gameState.notes,
                hiddenNotes: gameState.hiddenNotes,
                playerOrder: playerOrderWithNames,
                roundNumber: gameState.roundNumber,
            },
        };

        this.socket.emit('game:info', payload);
    }
}
