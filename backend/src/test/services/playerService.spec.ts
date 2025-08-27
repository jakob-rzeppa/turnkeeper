import { beforeEach, describe, expect, it } from "vitest";
import playerService, {
    Player,
    players,
} from "../../services/playerService.js";

describe("playerService", () => {
    beforeEach(() => {
        players.length = 0;
    });

    describe("getAllPlayers", () => {
        it("should return all players", () => {
            players.push({ name: "Alice", stats: [] });

            const result = playerService.getAllPlayers();

            expect(result).toHaveLength(1);
            expect(result[0].name).toBe("Alice");
        });

        it("should return an empty array if no players exist", () => {
            const result = playerService.getAllPlayers();
            expect(result).toHaveLength(0);
        });
    });

    describe("getPlayerByName", () => {
        it("should return a player by name", () => {
            players.push({ name: "Bob", stats: [] });

            const result = playerService.getPlayerByName("Bob");

            expect(result).toBeDefined();
            expect(result?.name).toBe("Bob");
        });

        it("should return null if player does not exist", () => {
            const result = playerService.getPlayerByName("NonExistentPlayer");
            expect(result).toBeNull();
        });
    });

    describe("addPlayer", () => {
        it("should add a player", () => {
            const newPlayer: Player = { name: "Charlie", stats: [] };

            playerService.addPlayer(newPlayer);

            expect(players).toHaveLength(1);
            expect(players[0]).toEqual(newPlayer);
        });
    });

    describe("updatePlayer", () => {
        it("should update a player", () => {
            const existingPlayer: Player = { name: "Bob", stats: [] };
            players.push(existingPlayer);

            const updatedPlayer: Player = {
                name: "UpdatedBob",
                stats: [
                    { name: "hp", value: 200 },
                    { name: "job", value: ["Warrior", "Mage"] },
                ],
            };

            playerService.updatePlayer("Bob", updatedPlayer);

            expect(players).toHaveLength(1);
            expect(players[0]).toEqual(updatedPlayer);
        });
    });

    describe("removePlayer", () => {
        it("should remove a player", () => {
            players.push({ name: "Bob", stats: [] });

            playerService.removePlayer("Bob");

            expect(players).toHaveLength(0);
        });
    });
});
