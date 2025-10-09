import GmGameEmitter from "../connectionEmitters/gm/GmGameEmitter.js";
import GmPlayersEmitter from "../connectionEmitters/gm/GmPlayersEmitter.js";
import playerRepository from "../repositories/playerRepository.js";
import { gameloop } from "./gameloop.js";

const playerHandler = {
    createPlayer(playerData: { name: string }) {
        playerRepository.createPlayer(playerData.name);
        GmPlayersEmitter.getInstance()?.sendPlayers();

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
        GmPlayersEmitter.getInstance()?.sendPlayers();

        GmGameEmitter.getInstance()?.sendGameInfo();
    },
    deletePlayer(playerId: string) {
        playerRepository.deletePlayer(playerId);
        GmPlayersEmitter.getInstance()?.sendPlayers();

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
        GmPlayersEmitter.getInstance()?.sendPlayers();
    },
    createStatForAllPlayers(statData: {
        name: string;
        value: boolean | number | string | string[];
    }) {
        playerRepository.createStatForAllPlayers(statData);
        GmPlayersEmitter.getInstance()?.sendPlayers();
    },
    removeStatFromPlayer({
        playerId,
        statName,
    }: {
        playerId: string;
        statName: string;
    }) {
        playerRepository.removeStatFromPlayer(playerId, statName);
        GmPlayersEmitter.getInstance()?.sendPlayers();
    },
};

export default playerHandler;
