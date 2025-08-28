import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import playerService from "../../services/playerService.js";
import statsService from "../../services/statsService.js";

describe("statsService", () => {
    beforeAll(() => {
        playerService.getAllPlayers = vi.fn();
        playerService.getPlayerByName = vi.fn();
        playerService.addPlayer = vi.fn();
        playerService.updatePlayer = vi.fn();
        playerService.removePlayer = vi.fn();
    });

    afterEach(() => {
        vi.clearAllMocks();
    });

    describe("addStatToAllPlayers", () => {
        it("should add a stat to all players", () => {
            const stat = { name: "goals", value: 1 };

            playerService.getAllPlayers = vi.fn().mockReturnValue([
                { name: "Player 1", stats: [] },
                { name: "Player 2", stats: [] },
            ]);

            statsService.addStatToAllPlayers(stat);

            expect(playerService.updatePlayer).toHaveBeenNthCalledWith(
                1,
                "Player 1",
                {
                    stats: [{ name: "goals", value: 1 }],
                }
            );
            expect(playerService.updatePlayer).toHaveBeenNthCalledWith(
                2,
                "Player 2",
                {
                    stats: [{ name: "goals", value: 1 }],
                }
            );
        });

        it("should add a stat only the players that don't have it yet", () => {
            const stat = { name: "goals", value: 1 };

            playerService.getAllPlayers = vi.fn().mockReturnValue([
                { name: "Player 1", stats: [{ name: "goals", value: 2 }] },
                { name: "Player 2", stats: [] },
            ]);

            statsService.addStatToAllPlayers(stat);

            expect(playerService.updatePlayer).toHaveBeenCalledTimes(1);
            expect(playerService.updatePlayer).toHaveBeenCalledWith(
                "Player 2",
                {
                    stats: [{ name: "goals", value: 1 }],
                }
            );
        });
    });

    describe("addStatToPlayer", () => {
        it("should add a stat to a specific player", () => {
            const stat = { name: "goals", value: 1 };

            playerService.getPlayerByName = vi.fn().mockReturnValue({
                name: "Player 1",
                stats: [],
            });

            statsService.addStatToPlayer("Player 1", stat);

            expect(playerService.updatePlayer).toHaveBeenCalledWith(
                "Player 1",
                {
                    stats: [{ name: "goals", value: 1 }],
                }
            );
        });

        it("shouldn't add a stat if the player doesn't exist", () => {
            const stat = { name: "goals", value: 1 };

            playerService.getPlayerByName = vi.fn().mockReturnValue(null);

            statsService.addStatToPlayer("Player 1", stat);

            expect(playerService.updatePlayer).not.toHaveBeenCalled();
        });

        it("shouldn't add a stat if the player already has the stat", () => {
            const stat = { name: "goals", value: 1 };

            playerService.getPlayerByName = vi.fn().mockReturnValue({
                name: "Player 1",
                stats: [{ name: "goals", value: 2 }],
            });

            statsService.addStatToPlayer("Player 1", stat);

            expect(playerService.updatePlayer).not.toHaveBeenCalled();
        });
    });

    describe("updateStatOfAllPlayers", () => {
        it("should update a stat for all players", () => {
            const stat = { name: "goals", value: 3 };

            playerService.getAllPlayers = vi.fn().mockReturnValue([
                { name: "Player 1", stats: [{ name: "goals", value: 2 }] },
                { name: "Player 2", stats: [{ name: "goals", value: 1 }] },
            ]);

            statsService.updateStatOfAllPlayers(stat);

            expect(playerService.updatePlayer).toHaveBeenCalledTimes(2);
            expect(playerService.updatePlayer).toHaveBeenNthCalledWith(
                1,
                "Player 1",
                {
                    stats: [{ name: "goals", value: 3 }],
                }
            );
            expect(playerService.updatePlayer).toHaveBeenNthCalledWith(
                2,
                "Player 2",
                {
                    stats: [{ name: "goals", value: 3 }],
                }
            );
        });

        it("shouldn't update a stat if the player doesn't have it", () => {
            const stat = { name: "goals", value: 3 };

            playerService.getAllPlayers = vi.fn().mockReturnValue([
                { name: "Player 1", stats: [{ name: "assists", value: 2 }] },
                { name: "Player 2", stats: [{ name: "goals", value: 1 }] },
            ]);

            statsService.updateStatOfAllPlayers(stat);

            expect(playerService.updatePlayer).toHaveBeenCalledTimes(1);
            expect(playerService.updatePlayer).toHaveBeenCalledWith(
                "Player 2",
                {
                    stats: [{ name: "goals", value: 3 }],
                }
            );
        });
    });

    describe("updateStatOfPlayer", () => {
        it("should update a stat for a specific player", () => {
            const stat = { name: "goals", value: 3 };

            playerService.getPlayerByName = vi.fn().mockReturnValue({
                name: "Player 1",
                stats: [{ name: "goals", value: 2 }],
            });

            statsService.updateStatOfPlayer("Player 1", stat);

            expect(playerService.updatePlayer).toHaveBeenCalledWith(
                "Player 1",
                {
                    stats: [{ name: "goals", value: 3 }],
                }
            );
        });

        it("shouldn't update a stat if the player doesn't exist", () => {
            const stat = { name: "goals", value: 3 };

            playerService.getPlayerByName = vi.fn().mockReturnValue(null);

            statsService.updateStatOfPlayer("Player 1", stat);

            expect(playerService.updatePlayer).not.toHaveBeenCalled();
        });

        it("shouldn't update a stat if the player doesn't have it", () => {
            const stat = { name: "goals", value: 3 };

            playerService.getPlayerByName = vi.fn().mockReturnValue({
                name: "Player 1",
                stats: [{ name: "assists", value: 2 }],
            });

            statsService.updateStatOfPlayer("Player 1", stat);

            expect(playerService.updatePlayer).not.toHaveBeenCalled();
        });
    });

    describe("removeStatFromPlayer", () => {
        it("should remove a stat from a specific player", () => {
            playerService.getPlayerByName = vi.fn().mockReturnValue({
                name: "Player 1",
                stats: [
                    { name: "goals", value: 2 },
                    { name: "assists", value: 1 },
                ],
            });

            statsService.removeStatFromPlayer("Player 1", "goals");

            expect(playerService.updatePlayer).toHaveBeenCalledWith(
                "Player 1",
                {
                    stats: [{ name: "assists", value: 1 }],
                }
            );
        });
    });
});
