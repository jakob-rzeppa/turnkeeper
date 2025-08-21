import { beforeEach, describe, expect, it } from "vitest";

import statsService, {
    PlayerStatValueTypes,
    stats,
} from "../../services/statsService.js";

describe("statsService", () => {
    beforeEach(() => {
        // Reset the stats Map before each test
        stats.clear();
    });

    describe("getStats", () => {
        it("should return an empty Map for a player with no stats", () => {
            const result = statsService.getStats("nonExistentPlayer");
            expect(result).toEqual(new Map<string, PlayerStatValueTypes>());
        });

        it("should return a Map of stats for an existing player", () => {
            const firstPlayerName = "existingPlayer";
            const firstPlayerStats = new Map<string, PlayerStatValueTypes>([
                ["score", 100],
                ["lives", 3],
                ["powerUps", ["speed", "shield"]],
            ]);
            stats.set(firstPlayerName, firstPlayerStats);
            const secondPlayerName = "anotherPlayer";
            const secondPlayerStats = new Map<string, PlayerStatValueTypes>([
                ["score", 200],
                ["lives", 5],
                ["powerUps", ["invisibility"]],
            ]);
            stats.set(secondPlayerName, secondPlayerStats);

            const result = statsService.getStatsForAllPlayers();

            expect(result).toEqual(
                new Map<string, Map<string, PlayerStatValueTypes>>([
                    [firstPlayerName, firstPlayerStats],
                    [secondPlayerName, secondPlayerStats],
                ])
            );
        });

        it("should return deep copy of the stats", () => {
            const firstPlayerName = "existingPlayer";
            const firstPlayerStats = new Map<string, PlayerStatValueTypes>([
                ["score", 100],
                ["lives", 3],
                ["powerUps", ["speed", "shield"]],
            ]);
            stats.set(firstPlayerName, firstPlayerStats);
            const secondPlayerName = "anotherPlayer";
            const secondPlayerStats = new Map<string, PlayerStatValueTypes>([
                ["score", 200],
                ["lives", 5],
                ["powerUps", ["invisibility"]],
            ]);
            stats.set(secondPlayerName, secondPlayerStats);

            const result = statsService.getStatsForAllPlayers();

            result.get(firstPlayerName)?.set("score", 300);
            expect(firstPlayerStats.get("score")).toBe(100);

            result.get(secondPlayerName)?.set("score", 400);
            expect(secondPlayerStats.get("score")).toBe(200);

            (result.get(firstPlayerName)?.get("powerUps") as string[])?.push(
                "invisibility"
            );
            expect(firstPlayerStats.get("powerUps")).toEqual([
                "speed",
                "shield",
            ]);

            (result.get(secondPlayerName)?.get("powerUps") as string[])?.push(
                "speed"
            );
            expect(secondPlayerStats.get("powerUps")).toEqual(["invisibility"]);

            expect(result).not.toEqual(
                new Map<string, Map<string, PlayerStatValueTypes>>([
                    [firstPlayerName, firstPlayerStats],
                    [secondPlayerName, secondPlayerStats],
                ])
            );
        });
    });

    describe("getStatsForAllPlayers", () => {
        it("should return an empty Map for no players", () => {
            const result = statsService.getStatsForAllPlayers();
            expect(result).toEqual(
                new Map<string, Map<string, PlayerStatValueTypes>>()
            );
        });

        it("should return a Map with all players and their corresponding stats", () => {
            const playerName = "existingPlayer";
            const playerStats = new Map<string, PlayerStatValueTypes>([
                ["score", 100],
                ["lives", 3],
                ["powerUps", ["speed", "shield"]],
            ]);
            stats.set(playerName, playerStats);

            const result = statsService.getStatsForAllPlayers();

            expect(result).toEqual(
                new Map<string, Map<string, PlayerStatValueTypes>>([
                    [playerName, playerStats],
                ])
            );
        });
    });

    describe("updateStat", () => {
        it("should update an existing stat for a player", () => {
            const playerName = "existingPlayer";
            const stat = "score";
            const newValue = 150;

            stats.set(playerName, new Map([[stat, 100]]));

            statsService.updateStat(playerName, stat, newValue);

            const result = stats.get(playerName);
            expect(result?.get(stat)).toBe(newValue);
        });

        it("should add a new stat for a player if it doesn't exist", () => {
            const playerName = "newPlayer";
            const stat = "score";
            const value = 200;

            statsService.updateStat(playerName, stat, value);

            const result = stats.get(playerName);
            expect(result?.get(stat)).toBe(value);
        });
    });

    describe("addStat", () => {
        it("should update an existing stat for a player", () => {
            const playerName = "existingPlayer";
            const stat = "score";
            const newValue = 150;

            stats.set(playerName, new Map([[stat, 100]]));

            statsService.addStat(playerName, stat, newValue);

            const result = stats.get(playerName);
            expect(result?.get(stat)).toBe(newValue);
        });

        it("should add a new stat for a player if it doesn't exist", () => {
            const playerName = "newPlayer";
            const stat = "score";
            const value = 200;

            statsService.addStat(playerName, stat, value);

            const result = stats.get(playerName);
            expect(result?.get(stat)).toBe(value);
        });
    });

    describe("removeStat", () => {
        it("should remove a stat", () => {
            const playerName = "existingPlayer";
            const stat = "score";
            const value = 100;

            stats.set(playerName, new Map([[stat, value]]));

            statsService.removeStat(playerName, stat);

            const result = stats.get(playerName);
            expect(result?.has(stat)).toBe(false);
        });
    });
});
