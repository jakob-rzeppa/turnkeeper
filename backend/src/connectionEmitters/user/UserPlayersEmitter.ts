import { BackendToUserPayloads } from 'shared-types';
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
        const player = playerRepository.getPlayerById(this.playerId);

        if (!player) {
            logger.error({
                details: { playerId: this.playerId },
                message: `Tried to emit player info for non-existing player`,
            });
            return;
        }

        const payload: BackendToUserPayloads['player:info'] = {
            player: {
                id: player.id,
                name: player.name,
                stats: player.stats,
            },
        };

        this.socket.emit('player:info', payload);
    }
}
