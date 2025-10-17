import GmController from "../connectionControllers/GmController";
import UserController from "../connectionControllers/UserController";
import { statsRepository } from "../repositories/statsRepository";

export const statsHandler = {
    createStatForAllPlayers(statData: {
        name: string;
        value: boolean | number | string | string[];
    }) {
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
        statData: { name: string; value: boolean | number | string | string[] };
    }) {
        statsRepository.createStatForPlayer(playerId, statData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(
            playerId
        )?.userPlayersEmitter.sendOwnPlayer();
    },
    removeStatFromPlayer({
        playerId,
        statId,
    }: {
        playerId: number;
        statId: number;
    }) {
        statsRepository.removeStatFromPlayer(playerId, statId);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(
            playerId
        )?.userPlayersEmitter.sendOwnPlayer();
    },
};
