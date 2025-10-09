import GmController from "../connectionControllers/GmController.js";
import playerRepository from "../repositories/playerRepository.js";
import { gameloop } from "./gameloop.js";

const playerHandler = {
    createPlayer(playerData: { name: string }) {
        playerRepository.createPlayer(playerData.name);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();

        // If the game loop is already initialized, add the new player to the turn order
        if (!gameloop.isInitialized()) {
            return;
        }

        const playerId = playerRepository.getPlayerIdByName(playerData.name);
        if (!playerId) {
            return;
        }

        gameloop.addPlayerToTurnOrder(playerId);
    },
    updatePlayer({
        playerId,
        playerData,
    }: {
        playerId: string;
        playerData: { name?: string; secret?: string };
    }) {
        playerRepository.updatePlayer(playerId, playerData);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
    deletePlayer(playerId: string) {
        playerRepository.deletePlayer(playerId);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();

        gameloop.removeDeletePlayersFromPlayerOrder();
    },
    createStatForPlayer({
        playerId,
        statData,
    }: {
        playerId: string;
        statData: { name: string; value: boolean | number | string | string[] };
    }) {
        playerRepository.createStatForPlayer(playerId, statData);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
    },
    createStatForAllPlayers(statData: {
        name: string;
        value: boolean | number | string | string[];
    }) {
        playerRepository.createStatForAllPlayers(statData);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
    },
    removeStatFromPlayer({
        playerId,
        statName,
    }: {
        playerId: string;
        statName: string;
    }) {
        playerRepository.removeStatFromPlayer(playerId, statName);
        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
    },
};

export default playerHandler;
