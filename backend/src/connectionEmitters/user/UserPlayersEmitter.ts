import { BackendToUserEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import playerRepository from '../../repositories/playerRepository.js';
import logger from '../../services/logger.js';

export default class UserPlayersEmitter {
    private playerId: number;
    private socket: Socket;

    public constructor(playerId: number, s: Socket) {
        this.playerId = playerId;
        this.socket = s;

        // Initial emits
        this.sendOwnPlayer();
    }

    public sendOwnPlayer() {
        try {
            const player = playerRepository.getPlayerById(this.playerId);

            const payload: BackendToUserEventPayloads['player:info'] = {
                player: {
                    id: player.id,
                    name: player.name,
                    notes: player.notes,
                    stats: player.stats,
                },
            };

            this.socket.emit('player:info', payload);
        } catch (error) {
            logger.error({ message: `Error fetching player with ID ${this.playerId}: ${error}` });
            return;
        }
    }
}
