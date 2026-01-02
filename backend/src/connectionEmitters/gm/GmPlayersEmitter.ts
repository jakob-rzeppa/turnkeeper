import { BackendToGmEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import playerRepository from '../../repositories/playerRepository.js';
import logger from '../../services/logger.js';

export default class GmPlayersEmitter {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        // Initial emits
        this.sendPlayers();
    }

    public sendPlayers() {
        try {
            const players = playerRepository.getAllPlayers();

            const payload: BackendToGmEventPayloads['players:info'] = { players };

            this.socket.emit('players:info', payload);
        } catch (error) {
            logger.error({ message: `Error fetching all players: ${error}` });
            return;
        }
    }
}
