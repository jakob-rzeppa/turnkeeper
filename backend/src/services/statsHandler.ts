import { PlayerStat } from 'shared-types';

import GmController from '../connectionControllers/GmController.js';
import UserController from '../connectionControllers/UserController.js';
import { statsRepository } from '../repositories/statsRepository.js';

export const statsHandler = {
    createStatForAllPlayers(statData: Omit<PlayerStat, 'id'>) {
        statsRepository.createStatForAllPlayers(statData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getAllInstances().forEach((instance) => {
            instance.userPlayersEmitter.sendOwnPlayer();
        });
    },
    createStatForPlayer({
        playerId,
        statData,
    }: {
        playerId: number;
        statData: Omit<PlayerStat, 'id'>;
    }) {
        statsRepository.createStatForPlayer(playerId, statData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer();
    },
    removeStat({
        playerId, // The playerId is used to update the correct user's view and to have a extra check that the changed stat belongs to the right player
        statId,
    }: {
        playerId: number;
        statId: number;
    }) {
        statsRepository.removeStatFromPlayer(playerId, statId);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer();
    },
    updateStat({
        newData,
        playerId, // The playerId is used to update the correct user's view and to have a extra check that the changed stat belongs to the right player
        statId,
    }: {
        newData: Partial<Omit<PlayerStat, 'id' | 'playerId'>>;
        playerId: number;
        statId: number;
    }) {
        statsRepository.updateStatForPlayer(playerId, statId, newData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer();
    },
};
