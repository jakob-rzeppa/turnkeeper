import { beforeEach, expect, test } from "vitest";
import playerService, { players } from "../../services/playerService.js";

beforeEach(() => {
    // Clear the players array before each test
    players.length = 0;
});

test("addPlayer", () => {
    playerService.addPlayer("Alice");

    expect(players).toHaveLength(1);
    expect(players[0]).toEqual({
        name: "Alice",
        isConnected: false,
        stats: new Map(),
    });
});

test("checkIfPlayerExists", () => {
    players.push({
        name: "Alice",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const exists = playerService.checkIfPlayerExists("Alice");
    expect(exists).toBe(true);

    const notExists = playerService.checkIfPlayerExists("Bob");
    expect(notExists).toBe(false);
});

test("getPlayer", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const actualPlayers = playerService.getPlayer("Bob");

    expect(actualPlayers).toEqual(
        expect.objectContaining({
            name: "Bob",
            isConnected: false,
        })
    );
});

test("getPlayer creates a deep copy", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>([
            ["hp", 200],
            ["jobs", ["warrior", "mage"]],
        ]),
    });

    const actualPlayer = playerService.getPlayer("Bob");

    expect(actualPlayer).not.toBeNull();

    players[0].name = "Bob2";
    expect(actualPlayer!.name).toBe("Bob");

    players[0].stats.set("hp", 100);
    expect(actualPlayer!.stats.get("hp")).toBe(200);

    const jobs = players[0].stats.get("jobs");
    (jobs as string[]).push("archer");
    (jobs as string[]).splice(0, 1);
    expect(actualPlayer!.stats.get("jobs")).toEqual(["warrior", "mage"]);
});

test("getPlayers", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>(),
    });
    players.push({
        name: "Bob2",
        isConnected: true,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const actualPlayers = playerService.getPlayers();

    expect(actualPlayers).toHaveLength(2);
    expect(actualPlayers).toEqual(
        expect.arrayContaining([
            expect.objectContaining({ name: "Bob", isConnected: false }),
            expect.objectContaining({ name: "Bob2", isConnected: true }),
        ])
    );
});

test("getPlayers creates a deep copy", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>([
            ["hp", 200],
            ["jobs", ["warrior", "mage"]],
        ]),
    });

    const actualPlayers = playerService.getPlayers();

    players[0].name = "Bob2";
    expect(actualPlayers[0].name).toBe("Bob");

    players[0].stats.set("hp", 100);
    expect(actualPlayers[0].stats.get("hp")).toBe(200);

    const jobs = players[0].stats.get("jobs");
    (jobs as string[]).push("archer");
    (jobs as string[]).splice(0, 1);
    expect(actualPlayers[0].stats.get("jobs")).toEqual(["warrior", "mage"]);
});

test("checkPlayerConnection false", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const isConnected = playerService.checkPlayerConnection("Bob");
    expect(isConnected).toBe(false);
});

test("checkPlayerConnection true", () => {
    players.push({
        name: "Bob",
        isConnected: true,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const isConnected = playerService.checkPlayerConnection("Bob");
    expect(isConnected).toBe(true);
});

test("setConnection true", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    playerService.setConnection("Bob", true);

    expect(players[0].isConnected).toBe(true);
});

test("setConnection false", () => {
    players.push({
        name: "Bob",
        isConnected: true,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    playerService.setConnection("Bob", false);

    expect(players[0].isConnected).toBe(false);
});

test("removePlayer", () => {
    players.push({
        name: "Bob",
        isConnected: false,
        stats: new Map<string, string | string[] | number | boolean>([
            ["hp", 200],
            ["jobs", ["warrior", "mage"]],
        ]),
    });

    playerService.removePlayer("Bob");

    expect(players).toHaveLength(0);
});
