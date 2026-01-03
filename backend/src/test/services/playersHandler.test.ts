import { GameState } from 'shared-types';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import UserController from '../../connectionControllers/UserController.js';
import playerRepository from '../../repositories/playerRepository.js';
import { statsRepository } from '../../repositories/statsRepository.js';
import gameStateHandler from '../../services/gameStateHandler.js';
import playersHandler from '../../services/playersHandler.js';

// Mock the dependencies
vi.mock('../../repositories/playerRepository', () => ({
    default: {
        createPlayer: vi.fn(),
        deletePlayer: vi.fn(),
        getPlayerIdByName: vi.fn(),
        updatePlayer: vi.fn(),
    },
}));
vi.mock('../../repositories/statsRepository', () => ({
    statsRepository: {
        removeAllStatsFromPlayer: vi.fn(),
    },
}));
vi.mock('../../connectionControllers/GmController', () => ({
    default: {
        getInstance: vi.fn().mockReturnValue({
            gmGameEmitter: { sendGameInfo: vi.fn() },
            gmPlayersEmitter: { sendPlayers: vi.fn() },
        } as unknown as GmController),
    },
}));
vi.mock('../../connectionControllers/UserController', () => ({
    default: {
        getAllInstances: vi.fn().mockReturnValue([
            {
                userGameEmitter: { sendGameInfo: vi.fn() },
            },
        ]),
        getInstance: vi.fn().mockReturnValue({
            disconnect: vi.fn(),
            userGameEmitter: { sendGameInfo: vi.fn() },
            userPlayersEmitter: { sendOwnPlayer: vi.fn() },
        } as unknown as UserController),
    },
}));
vi.mock('../../services/gameStateHandler', () => ({
    default: {
        addPlayerToTurnOrder: vi.fn(),
        getGameState: vi.fn(),
        removeDeletedPlayersFromPlayerOrder: vi.fn(),
    },
}));

describe('playersHandler', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe('createPlayer', () => {
        it('should create a player in the repository', () => {
            const playerData = { name: 'Test Player' };

            playersHandler.createPlayer(playerData);

            expect(playerRepository.createPlayer).toHaveBeenCalledWith(playerData.name);
        });

        it('should add the new player to the game state turn order if initialized', () => {
            const playerData = { name: 'Test Player' };
            const mockPlayerId = 1;
            vi.mocked(playerRepository.getPlayerIdByName).mockReturnValueOnce(mockPlayerId);
            vi.mocked(gameStateHandler.getGameState).mockReturnValueOnce({} as GameState);

            playersHandler.createPlayer(playerData);

            expect(gameStateHandler.addPlayerToTurnOrder).toHaveBeenCalledWith(mockPlayerId);
        });

        it('should not add the new player to the game state turn order if not initialized', () => {
            const playerData = { name: 'Test Player' };
            vi.mocked(gameStateHandler.getGameState).mockReturnValueOnce(null);

            playersHandler.createPlayer(playerData);

            expect(gameStateHandler.addPlayerToTurnOrder).not.toHaveBeenCalled();
        });

        it('should not add the new player to the gameloop turn order if player ID not found', () => {
            const playerData = { name: 'Test Player' };
            vi.mocked(playerRepository.getPlayerIdByName).mockReturnValueOnce(null);
            vi.mocked(gameStateHandler.getGameState).mockReturnValueOnce({} as GameState);

            playersHandler.createPlayer(playerData);

            expect(gameStateHandler.addPlayerToTurnOrder).not.toHaveBeenCalled();
        });
    });

    describe('updatePlayerInfo', () => {
        it('should update player info in the repository', () => {
            const playerId = 1;
            const playerData = { name: 'Updated Name' };

            playersHandler.updatePlayerInfo({ playerData, playerId });

            expect(playerRepository.updatePlayer).toHaveBeenCalledWith(playerId, playerData);
        });

        it('should notify GM and user controllers after updating player info', () => {
            const playerId = 1;
            const playerData = { name: 'Updated Name' };

            playersHandler.updatePlayerInfo({ playerData, playerId });

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer,
            ).toHaveBeenCalled();
            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(UserController.getInstance).toHaveBeenCalledWith(playerId);
            UserController.getAllInstances().forEach((instance) => {
                expect(instance.userGameEmitter.sendGameInfo).toHaveBeenCalled();
            });
        });
    });

    describe('deletePlayer', () => {
        it('should delete player from the repository', () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(playerRepository.deletePlayer).toHaveBeenCalledWith(playerId);
        });

        it('should remove all stats from the player before deletion', () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(statsRepository.removeAllStatsFromPlayer).toHaveBeenCalledWith(playerId);
        });

        it('should notify GM controller and disconnect user after deleting player', () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(UserController.getInstance(playerId)?.disconnect).toHaveBeenCalled();
        });

        it('should remove deleted player from gameloop turn order', () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(gameStateHandler.removeDeletedPlayersFromPlayerOrder).toHaveBeenCalled();
        });
    });
});
