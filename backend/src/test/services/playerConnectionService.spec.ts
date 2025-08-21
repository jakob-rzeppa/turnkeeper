import { beforeEach, expect, test } from "vitest";
import { players } from "../../services/playerService.js";
import playerConnectionService from "../../services/playerConnectionService.js";

beforeEach(() => {
    // Clear the players array before each test
    players.length = 0;
});

test("getConnectionId", () => {
    players.push({
        name: "Bob",
        currentConnectionId: "123",
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const connectionId = playerConnectionService.getConnectionId("Bob");
    expect(connectionId).toBe("123");
});

test("getConnectionId null", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const connectionId = playerConnectionService.getConnectionId("Bob");
    expect(connectionId).toBe(null);
});

test("setConnection", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    playerConnectionService.setConnection("Bob", "123");

    expect(players[0].currentConnectionId).toBe("123");
});

test("setConnection no player", () => {
    expect(() =>
        playerConnectionService.setConnection("Bob", "123")
    ).toThrowError("Player Bob does not exist");
});

test("setConnection already connected", () => {
    players.push({
        name: "Bob",
        currentConnectionId: "222",
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    expect(() =>
        playerConnectionService.setConnection("Bob", "123")
    ).toThrowError("Player Bob is already connected");
});

test("removeConnection", () => {
    players.push({
        name: "Bob",
        currentConnectionId: "222",
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    playerConnectionService.removeConnection("Bob");

    expect(players[0].currentConnectionId).toBe(null);
});

test("checkIfPlayerAlreadyConnected true", () => {
    players.push({
        name: "Bob",
        currentConnectionId: "222",
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const isConnected =
        playerConnectionService.checkIfPlayerAlreadyConnected("Bob");
    expect(isConnected).toBe(true);
});

test("checkIfPlayerAlreadyConnected false", () => {
    players.push({
        name: "Bob",
        currentConnectionId: null,
        stats: new Map<string, string | string[] | number | boolean>(),
    });

    const isConnected =
        playerConnectionService.checkIfPlayerAlreadyConnected("Bob");
    expect(isConnected).toBe(false);
});
