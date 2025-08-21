import { describe, it, expect, beforeEach } from "vitest";
import playerConnectionService, {
    connections,
} from "../../services/playerConnectionService.js";

// Mock Socket object
function createMockSocket(id: string) {
    return { id } as any;
}

describe("playerConnectionService", () => {
    beforeEach(() => {
        connections.clear();
    });

    it("setConnection should add a socket for a player", () => {
        const socket = createMockSocket("socket1");

        playerConnectionService.setConnection("alice", socket);

        expect(connections.get("alice")).toBe(socket);
    });

    it("getConnectionId should return the socket id if connected", () => {
        const socket = createMockSocket("socket2");
        connections.set("bob", socket);

        expect(playerConnectionService.getConnectionId("bob")).toBe("socket2");
    });

    it("getConnectionId should return null if not connected", () => {
        expect(playerConnectionService.getConnectionId("charlie")).toBeNull();
    });

    it("checkIfPlayerAlreadyConnected should return true if player is connected", () => {
        const socket = createMockSocket("socket3");
        connections.set("dave", socket);

        expect(
            playerConnectionService.checkIfPlayerAlreadyConnected("dave")
        ).toBe(true);
    });

    it("checkIfPlayerAlreadyConnected should return false if player is not connected", () => {
        expect(
            playerConnectionService.checkIfPlayerAlreadyConnected("eve")
        ).toBe(false);
    });

    it("checkIfPlayerAlreadyConnected should return false if player is set to null", () => {
        connections.set("frank", createMockSocket("socket4"));

        playerConnectionService.removeConnection("frank");

        expect(connections.get("frank")).toBeNull();
    });

    it("removeConnection should set the player connection to null", () => {
        connections.set("grace", createMockSocket("socket5"));

        playerConnectionService.removeConnection("grace");

        expect(connections.get("grace")).toBeNull();
    });
});
