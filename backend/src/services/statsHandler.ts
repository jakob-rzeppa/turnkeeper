import { PlayerStat } from 'shared-types';

import GmController from '../connectionControllers/GmController';
import UserController from '../connectionControllers/UserController';
import { statsRepository } from '../repositories/statsRepository';

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
    updateStatValue({
        newValue,
        playerId, // The playerId is used to update the correct user's view and to have a extra check that the changed stat belongs to the right player
        statId,
    }: {
        newValue: PlayerStat['value'];
        playerId: number;
        statId: number;
    }) {
        statsRepository.updateStatForPlayer(playerId, statId, {
            value: newValue,
        });

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer();
    },
};
