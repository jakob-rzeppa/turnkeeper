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
        currentConnectionId: null,
        stats: new Map(),
    });
});

test("getPlayer", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const actualPlayers = playerService.getPlayer("Bob");

    expect(actualPlayers).toEqual(
        expect.objectContaining({
            name: "Bob",
            currentConnectionId: null,
        })
    );
});

test("getPlayer creates a deep copy", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
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
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });
    players.push({
        name: "Bob2",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const actualPlayers = playerService.getPlayers();

    expect(actualPlayers).toHaveLength(2);
    expect(actualPlayers).toEqual(
        expect.arrayContaining([
            expect.objectContaining({ name: "Bob", currentConnectionId: null }),
            expect.objectContaining({
                name: "Bob2",
                currentConnectionId: null,
            }),
        ])
    );
});

test("getPlayers creates a deep copy", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
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

test("removePlayer", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>([
            ["hp", 200],
            ["jobs", ["warrior", "mage"]],
        ]),
    });

    playerService.removePlayer("Bob");

    expect(players).toHaveLength(0);
});
