import { Player } from "shared-types";

import GmController from "../connectionControllers/GmController.js";
import UserController from "../connectionControllers/UserController.js";
import playerRepository from "../repositories/playerRepository.js";
import { statsRepository } from "../repositories/statsRepository.js";
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
    deletePlayer(playerId: number) {
        playerRepository.deletePlayer(playerId);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.disconnect();

        gameloop.removeDeletePlayersFromPlayerOrder();
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
    updatePlayerInfo({
        playerData,
        playerId,
    }: {
        playerData: Partial<Omit<Player, "id" | "stats">>;
        playerId: number;
    }) {
        playerRepository.updatePlayer(playerId, playerData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(
            playerId
        )?.userPlayersEmitter.sendOwnPlayer();

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
    },
};

export default playerHandler;
