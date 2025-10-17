import { Player } from "shared-types";

import GmController from "../connectionControllers/GmController.js";
import UserController from "../connectionControllers/UserController.js";
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
        UserController.getAllInstances().forEach((userController) => {
            userController.userGameEmitter.sendGameInfo();
        });
    },
    deletePlayer(playerId: number) {
        playerRepository.deletePlayer(playerId);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.disconnect();

        gameloop.removeDeletePlayersFromPlayerOrder();
    },
};

export default playerHandler;
