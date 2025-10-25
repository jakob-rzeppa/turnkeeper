import { GmToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';

import playerHandler from '../../services/playersHandler.js';
import { statsHandler } from '../../services/statsHandler.js';

export default class GmPlayersListener {
    private socket: Socket;

    public constructor(s: Socket) {
        this.socket = s;

        this.socket.on(
            'players:create',
            (playerData: GmToBackendEventPayloads['players:create']) => {
                playerHandler.createPlayer(playerData);
            },
        );

        this.socket.on(
            'players:update',
            ({ playerData, playerId }: GmToBackendEventPayloads['players:update']) => {
                playerHandler.updatePlayerInfo({ playerData, playerId });
            },
        );

        this.socket.on(
            'players:delete',
            ({ playerId }: GmToBackendEventPayloads['players:delete']) => {
                playerHandler.deletePlayer(playerId);
            },
        );

        this.socket.on(
            'players:stats:create',
            ({ playerId, scope, statData }: GmToBackendEventPayloads['players:stats:create']) => {
                if (scope === 'player' && playerId) {
                    statsHandler.createStatForPlayer({ playerId, statData });
                } else {
                    statsHandler.createStatForAllPlayers(statData);
                }
            },
        );

        this.socket.on(
            'players:stats:update',
            ({
                playerId,
                statId,
                name,
                value,
            }: GmToBackendEventPayloads['players:stats:update']) => {
                statsHandler.updateStat({
                    newData: { name, value },
                    playerId,
                    statId,
                });
            },
        );

        this.socket.on(
            'players:stats:remove',
            ({ playerId, statId }: GmToBackendEventPayloads['players:stats:remove']) => {
                statsHandler.removeStat({ playerId, statId });
            },
        );
    }
}
