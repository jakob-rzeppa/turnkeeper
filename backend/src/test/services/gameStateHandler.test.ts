import { beforeEach, describe, expect, it, Mock, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import UserController from '../../connectionControllers/UserController.js';
import gameStateRepository from '../../repositories/gameStateRepository.js';
import playerRepository from '../../repositories/playerRepository.js';
import gameStateHandler from '../../services/gameStateHandler.js';
import logger from '../../services/logger.js';

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;

vi.mock('../../repositories/gameStateRepository', () => {
    return {
        default: {
            advanceTurn: vi.fn(),
            revertTurn: vi.fn(),
            createGameState: vi.fn(),
            deleteGameState: vi.fn(),
            getGameStateById: vi.fn(),
            updateHiddenNotes: vi.fn(),
            updateNotes: vi.fn(),
            updatePlayerOrder: vi.fn(),
            addPlayerToOrder: vi.fn(),
            removePlayerFromOrder: vi.fn(),
        },
    };
});

vi.mock('../../repositories/playerRepository', () => {
    return {
        default: {
            getAllPlayers: vi.fn(),
            getPlayerNameById: vi.fn(),
        },
    };
});

vi.mock('../../services/logger', () => {
    return {
        default: {
            warn: vi.fn(),
            error: vi.fn(),
        },
    };
});

vi.mock('../../connectionControllers/GmController', () => {
    return {
        default: {
            getInstance: vi.fn().mockReturnValue({
                gmGameEmitter: {
                    sendGameInfo: vi.fn(),
                },
            }),
        },
    };
});

vi.mock('../../connectionControllers/UserController', () => {
    return {
        default: {
            getAllInstances: vi.fn().mockReturnValue([
                {
                    userGameEmitter: {
                        sendGameInfo: vi.fn(),
                    },
                },
            ]),
        },
    };
});

describe('gameStateHandler', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe('getGameState', () => {
        it('should retrieve the game state from the repository', () => {
            const mockGameState = {
                id: GAME_STATE_ID,
                currentPlayerIndex: 1,
                roundNumber: 2,
                notes: 'Test notes',
                hiddenNotes: 'Hidden notes',
                playerOrder: [1, 2, 3],
            };
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(mockGameState);

            const result = gameStateHandler.getGameState();

            expect(gameStateRepository.getGameStateById).toHaveBeenCalledWith(GAME_STATE_ID);
            expect(result).toBe(mockGameState);
        });

        it('should return null', () => {
            vi.mocked(gameStateRepository.getGameStateById).mockImplementation(() => {
                throw new Error('Database error');
            });

            const result = gameStateHandler.getGameState();

            expect(gameStateRepository.getGameStateById).toHaveBeenCalledWith(GAME_STATE_ID);
            expect(result).toBeNull();
        });
    });

    describe('initGameState', () => {
        it('should create a new game state with the given player order', () => {
            const playerOrder = [3, 1, 2];

            gameStateHandler.initGameState(playerOrder);

            expect(gameStateRepository.createGameState).toHaveBeenCalledWith(playerOrder);
        });

        it('should log an error if creation fails', () => {
            const playerOrder = [3, 1, 2];
            vi.mocked(gameStateRepository.createGameState).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.initGameState(playerOrder);

            expect(gameStateRepository.createGameState).toHaveBeenCalledWith(playerOrder);
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to initialize game state: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            const playerOrder = [3, 1, 2];

            gameStateHandler.initGameState(playerOrder);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('deleteGameState', () => {
        it('should delete the current game state', () => {
            gameStateHandler.deleteGameState();

            expect(gameStateRepository.deleteGameState).toHaveBeenCalled();
        });

        it('should log an error if deletion fails', () => {
            vi.mocked(gameStateRepository.deleteGameState).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.deleteGameState();

            expect(gameStateRepository.deleteGameState).toHaveBeenCalled();
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to delete game state: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            gameStateHandler.deleteGameState();

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('advanceTurn', () => {
        it('should advance the turn to the next player', () => {
            gameStateHandler.advanceTurn();

            expect(gameStateRepository.advanceTurn).toHaveBeenCalled();
        });

        it('should log an error if advancing the turn fails', () => {
            vi.mocked(gameStateRepository.advanceTurn).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.advanceTurn();

            expect(gameStateRepository.advanceTurn).toHaveBeenCalled();
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to advance turn: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            gameStateHandler.advanceTurn();

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('updateNotes', () => {
        it('should update the public notes of the game state', () => {
            const newNotes = 'Updated public notes';

            gameStateHandler.updateNotes(newNotes);

            expect(gameStateRepository.updateNotes).toHaveBeenCalledWith(GAME_STATE_ID, newNotes);
        });

        it('should log an error if updating notes fails', () => {
            const newNotes = 'Updated public notes';
            vi.mocked(gameStateRepository.updateNotes).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.updateNotes(newNotes);

            expect(gameStateRepository.updateNotes).toHaveBeenCalledWith(GAME_STATE_ID, newNotes);
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to update public notes: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            const newNotes = 'Updated public notes';

            gameStateHandler.updateNotes(newNotes);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('updateHiddenNotes', () => {
        it('should update the hidden notes of the game state', () => {
            const newHiddenNotes = 'Updated hidden notes';

            gameStateHandler.updateHiddenNotes(newHiddenNotes);

            expect(gameStateRepository.updateHiddenNotes).toHaveBeenCalledWith(
                GAME_STATE_ID,
                newHiddenNotes,
            );
        });

        it('should log an error if updating hidden notes fails', () => {
            const newHiddenNotes = 'Updated hidden notes';
            vi.mocked(gameStateRepository.updateHiddenNotes).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.updateHiddenNotes(newHiddenNotes);

            expect(gameStateRepository.updateHiddenNotes).toHaveBeenCalledWith(
                GAME_STATE_ID,
                newHiddenNotes,
            );
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to update hidden notes: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            const newHiddenNotes = 'Updated hidden notes';

            gameStateHandler.updateHiddenNotes(newHiddenNotes);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('updatePlayerOrder', () => {
        it('should update the player order in the game state', () => {
            const newPlayerOrder = [3, 1, 2];

            gameStateHandler.updatePlayerOrder(newPlayerOrder);

            expect(gameStateRepository.updatePlayerOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                newPlayerOrder,
            );
        });

        it('should log an error if updating player order fails', () => {
            const newPlayerOrder = [3, 1, 2];
            vi.mocked(gameStateRepository.updatePlayerOrder).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.updatePlayerOrder(newPlayerOrder);

            expect(gameStateRepository.updatePlayerOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                newPlayerOrder,
            );
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to update player order: Database error',
            });
        });

        it('should send game info to GM and all users', () => {
            const newPlayerOrder = [3, 1, 2];

            gameStateHandler.updatePlayerOrder(newPlayerOrder);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('addPlayerToTurnOrder', () => {
        it('should add a new player to the turn order', () => {
            const playerId = 4;

            gameStateHandler.addPlayerToTurnOrder(playerId);

            expect(gameStateRepository.addPlayerToOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
        });

        it('should log an error if adding player to turn order fails', () => {
            const playerId = 4;
            vi.mocked(gameStateRepository.addPlayerToOrder).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.addPlayerToTurnOrder(playerId);

            expect(gameStateRepository.addPlayerToOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
            expect(logger.error).toHaveBeenCalledWith({
                message: `Failed to add player ${playerId} to turn order: Database error`,
            });
        });

        it('should send game info to GM and all users', () => {
            const playerId = 4;

            gameStateHandler.addPlayerToTurnOrder(playerId);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });

    describe('removePlayerFromTurnOrder', () => {
        it('should remove a player from the turn order', () => {
            const playerId = 2;

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(gameStateRepository.removePlayerFromOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
        });

        it('should update current player index if the removed player is the current player', () => {
            const playerId = 2;
            const mockGameState = {
                id: GAME_STATE_ID,
                currentPlayerIndex: 1,
                roundNumber: 2,
                notes: 'Test notes',
                hiddenNotes: 'Hidden notes',
                playerOrder: [1, 2, 3],
            };
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(mockGameState);

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(gameStateRepository.removePlayerFromOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
            expect(gameStateRepository.revertTurn).toHaveBeenCalled();
        });

        it('should update current player index if the removed player is before the current player', () => {
            const playerId = 1;
            const mockGameState = {
                id: GAME_STATE_ID,
                currentPlayerIndex: 1,
                roundNumber: 2,
                notes: 'Test notes',
                hiddenNotes: 'Hidden notes',
                playerOrder: [1, 2, 3],
            };
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(mockGameState);

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(gameStateRepository.removePlayerFromOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
            expect(gameStateRepository.revertTurn).toHaveBeenCalled();
        });

        it('should log an error if removing player from turn order fails', () => {
            const playerId = 2;
            vi.mocked(gameStateRepository.removePlayerFromOrder).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(gameStateRepository.removePlayerFromOrder).toHaveBeenCalledWith(
                GAME_STATE_ID,
                playerId,
            );
            expect(logger.error).toHaveBeenCalledWith({
                message: 'Failed to remove player 2 from turn order: Database error',
            });
        });

        it('should not attempt to update current player index if game state retrieval fails', () => {
            const playerId = 2;
            vi.mocked(gameStateRepository.getGameStateById).mockImplementation(() => {
                throw new Error('Database error');
            });

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(gameStateRepository.revertTurn).not.toHaveBeenCalled();
        });

        it('should send game info to GM and all users', () => {
            const playerId = 2;

            gameStateHandler.removePlayerFromTurnOrder(playerId);

            expect(GmController.getInstance()?.gmGameEmitter.sendGameInfo).toHaveBeenCalled();
            expect(
                UserController.getAllInstances()[0].userGameEmitter.sendGameInfo,
            ).toHaveBeenCalled();
        });
    });
});
