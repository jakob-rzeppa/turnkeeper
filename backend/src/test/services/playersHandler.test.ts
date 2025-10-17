import { beforeEach, describe, expect, it, vi } from "vitest";

import GmController from "../../connectionControllers/GmController.js";
import UserController from "../../connectionControllers/UserController.js";
import playerRepository from "../../repositories/playerRepository.js";
import { gameloop } from "../../services/gameloop.js";
import playersHandler from "../../services/playersHandler.js";
import { disconnect } from "process";

// Mock the dependencies
vi.mock("../../repositories/playerRepository", () => ({
    default: {
        createPlayer: vi.fn(),
        getPlayerIdByName: vi.fn(),
        updatePlayer: vi.fn(),
        deletePlayer: vi.fn(),
    },
}));
vi.mock("../../connectionControllers/GmController", () => ({
    default: {
        getInstance: vi.fn().mockReturnValue({
            gmPlayersEmitter: { sendPlayers: vi.fn() },
            gmGameEmitter: { sendGameInfo: vi.fn() },
        } as unknown as GmController),
    },
}));
vi.mock("../../connectionControllers/UserController", () => ({
    default: {
        getInstance: vi.fn().mockReturnValue({
            userPlayersEmitter: { sendOwnPlayer: vi.fn() },
            userGameEmitter: { sendGameInfo: vi.fn() },
            disconnect: vi.fn(),
        } as unknown as UserController),
        getAllInstances: vi.fn().mockReturnValue([
            {
                userGameEmitter: { sendGameInfo: vi.fn() },
            },
        ]),
    },
}));
vi.mock("../../services/gameloop", () => ({
    gameloop: {
        isInitialized: vi.fn().mockReturnValue(true),
        addPlayerToTurnOrder: vi.fn(),
        removeDeletePlayersFromPlayerOrder: vi.fn(),
    },
}));

describe("playersHandler", () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe("createPlayer", () => {
        it("should create a player in the repository", () => {
            const playerData = { name: "Test Player" };

            playersHandler.createPlayer(playerData);

            expect(playerRepository.createPlayer).toHaveBeenCalledWith(
                playerData.name
            );
        });

        it("should add the new player to the gameloop turn order if initialized", () => {
            const playerData = { name: "Test Player" };
            const mockPlayerId = 1;
            vi.mocked(playerRepository.getPlayerIdByName).mockReturnValueOnce(
                mockPlayerId
            );
            vi.mocked(gameloop.isInitialized).mockReturnValueOnce(true);

            playersHandler.createPlayer(playerData);

            expect(gameloop.addPlayerToTurnOrder).toHaveBeenCalledWith(
                mockPlayerId
            );
        });

        it("should not add the new player to the gameloop turn order if not initialized", () => {
            const playerData = { name: "Test Player" };
            vi.mocked(gameloop.isInitialized).mockReturnValueOnce(false);

            playersHandler.createPlayer(playerData);

            expect(gameloop.addPlayerToTurnOrder).not.toHaveBeenCalled();
        });

        it("should not add the new player to the gameloop turn order if player ID not found", () => {
            const playerData = { name: "Test Player" };
            vi.mocked(playerRepository.getPlayerIdByName).mockReturnValueOnce(
                null
            );
            vi.mocked(gameloop.isInitialized).mockReturnValueOnce(true);

            playersHandler.createPlayer(playerData);

            expect(gameloop.addPlayerToTurnOrder).not.toHaveBeenCalled();
        });
    });

    describe("updatePlayerInfo", () => {
        it("should update player info in the repository", () => {
            const playerId = 1;
            const playerData = { name: "Updated Name" };

            playersHandler.updatePlayerInfo({ playerId, playerData });

            expect(playerRepository.updatePlayer).toHaveBeenCalledWith(
                playerId,
                playerData
            );
        });

        it("should notify GM and user controllers after updating player info", () => {
            const playerId = 1;
            const playerData = { name: "Updated Name" };

            playersHandler.updatePlayerInfo({ playerId, playerData });

            expect(
                GmController.getInstance()?.gmPlayersEmitter.sendPlayers
            ).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.userPlayersEmitter
                    .sendOwnPlayer
            ).toHaveBeenCalled();
            expect(
                GmController.getInstance()?.gmGameEmitter.sendGameInfo
            ).toHaveBeenCalled();
            expect(UserController.getInstance).toHaveBeenCalledWith(playerId);
            UserController.getAllInstances().forEach((instance) => {
                expect(
                    instance.userGameEmitter.sendGameInfo
                ).toHaveBeenCalled();
            });
        });
    });

    describe("deletePlayer", () => {
        it("should delete player from the repository", () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(playerRepository.deletePlayer).toHaveBeenCalledWith(
                playerId
            );
        });

        it("should notify GM controller and disconnect user after deleting player", () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(
                GmController.getInstance()?.gmPlayersEmitter.sendPlayers
            ).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.disconnect
            ).toHaveBeenCalled();
        });

        it("should remove deleted player from gameloop turn order", () => {
            const playerId = 1;

            playersHandler.deletePlayer(playerId);

            expect(
                gameloop.removeDeletePlayersFromPlayerOrder
            ).toHaveBeenCalled();
        });
    });
});
