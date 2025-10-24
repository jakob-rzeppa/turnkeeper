import { beforeEach, describe, expect, it, vi } from "vitest";
import gameStateHandler from "../../services/gameStateHandler";
import gameStateRepository from "../../repositories/gameStateRepository";
import logger from "../../services/logger";
import playerRepository from "../../repositories/playerRepository";

// Using a constant ID since for now there is only one game state at a time
const GAME_STATE_ID = 1;

vi.mock("../../repositories/gameStateRepository", () => {
    return {
        default: {
            getGameStateById: vi.fn(),
            createGameState: vi.fn(),
            deleteGameState: vi.fn(),
            updateGameState: vi.fn(),
        },
    };
});

vi.mock("../../repositories/playerRepository", () => {
    return {
        default: {
            getPlayerNameById: vi.fn(),
            getAllPlayers: vi.fn(),
        },
    };
});

vi.mock("../../services/logger", () => {
    return {
        default: {
            warn: vi.fn(),
        },
    };
});

describe("gameStateHandler", () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe("getGameState", () => {
        it("should return the current game state", () => {
            const expectedGameState = {
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                    { id: 3, name: "Charlie" },
                ],
            };

            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(
                expectedGameState
            );

            const gameState = gameStateHandler.getGameState();

            expect(gameState).toEqual(expectedGameState);
        });
    });

    describe("initGameState", () => {
        it("should initialize a new game state with the provided player order", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockImplementation(
                (id) => {
                    const names: Record<number, string> = {
                        1: "Alice",
                        2: "Bob",
                        3: "Charlie",
                    };
                    return names[id] || null;
                }
            );

            const newPlayerIdOrder = [1, 2, 3];

            gameStateHandler.initGameState(newPlayerIdOrder);

            expect(gameStateRepository.createGameState).toHaveBeenCalledWith(
                expect.objectContaining({
                    roundNumber: 1,
                    currentPlayerIndex: 0,
                    playerOrder: [
                        { id: 1, name: "Alice" },
                        { id: 2, name: "Bob" },
                        { id: 3, name: "Charlie" },
                    ],
                })
            );
        });

        it("should handle empty player order", () => {
            const newPlayerIdOrder: number[] = [];

            gameStateHandler.initGameState(newPlayerIdOrder);

            expect(playerRepository.getPlayerNameById).not.toHaveBeenCalled();
            expect(gameStateRepository.createGameState).toHaveBeenCalledWith(
                expect.objectContaining({
                    roundNumber: 1,
                    currentPlayerIndex: 0,
                    playerOrder: [],
                })
            );
        });

        it("should log a warning if any player IDs do not exist", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockImplementation(
                (id) => {
                    const names: Record<number, string> = {
                        1: "Alice",
                        2: "Bob",
                    };
                    return names[id] || null;
                }
            );

            const newPlayerIdOrder = [1, 2, 999]; // 999 does not exist

            gameStateHandler.initGameState(newPlayerIdOrder);

            expect(logger.warn).toHaveBeenCalledWith({
                message:
                    "Attempted to initialize game state with non-existing player IDs.",
            });
            expect(gameStateRepository.createGameState).not.toHaveBeenCalled();
        });
    });

    describe("deleteGameState", () => {
        it("should delete the current game state", () => {
            gameStateHandler.deleteGameState();

            expect(gameStateRepository.deleteGameState).toHaveBeenCalledWith(
                GAME_STATE_ID
            );
        });
    });

    describe("nextTurn", () => {
        it("should update the currentPlayerIndex to the next player", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });

            gameStateHandler.nextTurn();

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    currentPlayerIndex: 1,
                })
            );
        });

        it("should increment roundNumber and reset currentPlayerIndex when at end of player order", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 1,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });

            gameStateHandler.nextTurn();

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    roundNumber: 2,
                    currentPlayerIndex: 0,
                })
            );
        });

        it("should handle empty player order", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [],
            });

            gameStateHandler.nextTurn();

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    currentPlayerIndex: 0,
                    roundNumber: 2,
                })
            );
        });

        it("should log if no game state exists", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(
                null
            );

            gameStateHandler.nextTurn();

            expect(logger.warn).toHaveBeenCalledWith({
                message:
                    "No game state found when attempting to advance to next turn.",
            });
            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });
    });

    describe("addPlayerToTurnOrder", () => {
        it("should log a warning if the player does not exist", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockReturnValue(null);

            gameStateHandler.addPlayerToTurnOrder(999);

            expect(logger.warn).toHaveBeenCalledWith({
                message: `Player with ID 999 not found.`,
            });
            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });

        it("should log a warning if the game state does not exist", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockReturnValue(
                "Test Player"
            );
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(
                null
            );

            gameStateHandler.addPlayerToTurnOrder(1);

            expect(logger.warn).toHaveBeenCalledWith({
                message:
                    "No game state found when attempting to add player to turn order.",
            });
            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });

        it("should log a warning if the player is already in the turn order", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockReturnValue(
                "Alice"
            );
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });

            gameStateHandler.addPlayerToTurnOrder(1);

            expect(logger.warn).toHaveBeenCalledWith({
                message: `Player with ID 1 is already in the turn order.`,
            });
            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });

        it("should add the player to the turn order", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockReturnValue(
                "Charlie"
            );
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });

            gameStateHandler.addPlayerToTurnOrder(3);

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    playerOrder: [
                        { id: 1, name: "Alice" },
                        { id: 2, name: "Bob" },
                        { id: 3, name: "Charlie" },
                    ],
                })
            );
        });
    });

    describe("removeDeletedPlayersFromPlayerOrder", () => {
        it("should log a warning if no game state exists", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue(
                null
            );

            gameStateHandler.removeDeletedPlayersFromPlayerOrder();

            expect(logger.warn).toHaveBeenCalledWith({
                message:
                    "No game state found when attempting to remove deleted players from turn order.",
            });
        });

        it("should remove deleted players from the turn order", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                    { id: 3, name: "Charlie" },
                ],
            });
            vi.mocked(playerRepository.getAllPlayers).mockReturnValue([
                { id: 1, name: "Alice", secret: "secret1", stats: [] },
                { id: 2, name: "Bob", secret: "secret2", stats: [] },
            ]);

            gameStateHandler.removeDeletedPlayersFromPlayerOrder();

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    playerOrder: [
                        { id: 1, name: "Alice" },
                        { id: 2, name: "Bob" },
                    ],
                })
            );
        });

        it("should handle case where all players are deleted", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });
            vi.mocked(playerRepository.getAllPlayers).mockReturnValue([]);

            gameStateHandler.removeDeletedPlayersFromPlayerOrder();

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    playerOrder: [],
                })
            );
        });

        it("should not update if no players were deleted", () => {
            vi.mocked(gameStateRepository.getGameStateById).mockReturnValue({
                id: 1,
                roundNumber: 1,
                currentPlayerIndex: 0,
                playerOrder: [
                    { id: 1, name: "Alice" },
                    { id: 2, name: "Bob" },
                ],
            });
            vi.mocked(playerRepository.getAllPlayers).mockReturnValue([
                { id: 1, name: "Alice", secret: "secret1", stats: [] },
                { id: 2, name: "Bob", secret: "secret2", stats: [] },
            ]);

            gameStateHandler.removeDeletedPlayersFromPlayerOrder();

            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });
    });

    describe("updatePlayerOrder", () => {
        it("should update the player order in the game state", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockImplementation(
                (id) => {
                    const names: Record<number, string> = {
                        1: "Alice",
                        2: "Bob",
                        3: "Charlie",
                    };
                    return names[id] || null;
                }
            );
            const newPlayerOrder = [3, 2, 1];

            gameStateHandler.updatePlayerOrder(newPlayerOrder);

            expect(gameStateRepository.updateGameState).toHaveBeenCalledWith(
                GAME_STATE_ID,
                expect.objectContaining({
                    playerOrder: [
                        { id: 3, name: "Charlie" },
                        { id: 2, name: "Bob" },
                        { id: 1, name: "Alice" },
                    ],
                })
            );
        });

        it("should log a warning if any player IDs do not exist", () => {
            vi.mocked(playerRepository.getPlayerNameById).mockImplementation(
                (id) => {
                    const names: Record<number, string> = {
                        1: "Alice",
                        2: "Bob",
                    };
                    return names[id] || null;
                }
            );
            const newPlayerOrder = [1, 2, 999]; // 999 does not exist

            gameStateHandler.updatePlayerOrder(newPlayerOrder);

            expect(logger.warn).toHaveBeenCalledWith({
                message:
                    "Attempted to update player order with non-existing player IDs.",
            });
            expect(gameStateRepository.updateGameState).not.toHaveBeenCalled();
        });
    });
});
