import { Player } from 'shared-types';

import GmController from '../connectionControllers/GmController.js';
import UserController from '../connectionControllers/UserController.js';
import playerRepository from '../repositories/playerRepository.js';
import { statsRepository } from '../repositories/statsRepository.js';
import gameStateHandler from './gameStateHandler.js';

const playerHandler = {
    createPlayer(playerData: { name: string }) {
        playerRepository.createPlayer(playerData.name);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();

        // If the game loop is already initialized, add the new player to the turn order
        if (!gameStateHandler.getGameState()) {
            return;
        }
        try {
            const playerId = playerRepository.getPlayerIdByName(playerData.name);

            gameStateHandler.addPlayerToTurnOrder(playerId);
        } catch (error) {
            if (error instanceof Error) {
                console.error(`Failed to add new player to turn order: ${error.message}`);
            }
        }
    },
    deletePlayer(playerId: number) {
        statsRepository.removeAllStatsFromPlayer(playerId);
        playerRepository.deletePlayer(playerId);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.disconnect();

        gameStateHandler.removePlayerFromTurnOrder(playerId);
    },
    updatePlayerInfo({
        playerData,
        playerId,
    }: {
        playerData: Partial<Omit<Player, 'id' | 'stats'>>;
        playerId: number;
    }) {
        playerRepository.updatePlayer(playerId, playerData);

        GmController.getInstance()?.gmPlayersEmitter.sendPlayers();
        UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer();

        GmController.getInstance()?.gmGameEmitter.sendGameInfo();
        UserController.getAllInstances().forEach((userController) => {
            userController.userGameEmitter.sendGameInfo();
        });
    },
};

export default playerHandler;
