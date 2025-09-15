import { beforeEach, describe, expect, it } from "vitest";
import playerRepository, {
    Player,
    players,
} from "../../repositories/playerRepository.js";

describe("playerRepository", () => {
    beforeEach(() => {
        players.length = 0;
    });

    describe("getAllPlayers", () => {
        it("should return all players", () => {
            players.push({ name: "Alice", stats: [] });

            const result = playerRepository.getAllPlayers();

            expect(result).toHaveLength(1);
            expect(result[0].name).toBe("Alice");
        });

        it("should return an empty array if no players exist", () => {
            const result = playerRepository.getAllPlayers();
            expect(result).toHaveLength(0);
        });
    });

    describe("getPlayerByName", () => {
        it("should return a player by name", () => {
            players.push({ name: "Bob", stats: [] });

            const result = playerRepository.getPlayerByName("Bob");

            expect(result).toBeDefined();
            expect(result?.name).toBe("Bob");
        });

        it("should return null if player does not exist", () => {
            const result =
                playerRepository.getPlayerByName("NonExistentPlayer");
            expect(result).toBeNull();
        });
    });

    describe("addPlayer", () => {
        it("should add a player", () => {
            const newPlayer: Player = { name: "Charlie", stats: [] };

            playerRepository.addPlayer("Charlie");

            expect(players).toHaveLength(1);
            expect(players[0]).toEqual(newPlayer);
        });

        it("shouldn't add a player if a player with the same name exists", () => {
            const player = {
                name: "Alice",
                stats: [{ name: "hp", value: 200 }],
            };

            players.push(player);

            playerRepository.addPlayer("Alice");

            expect(players).toHaveLength(1);
            expect(players[0]).toEqual(player);
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

            playerRepository.updatePlayer("Bob", updatedPlayer);

            expect(players).toHaveLength(1);
            expect(players[0]).toEqual(updatedPlayer);
        });
    });

    describe("removePlayer", () => {
        it("should remove a player", () => {
            players.push({ name: "Bob", stats: [] });

            playerRepository.removePlayer("Bob");

            expect(players).toHaveLength(0);
        });
    });
});
