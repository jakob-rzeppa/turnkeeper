import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import {
    connection,
    initConnection,
    sendPlayerData,
} from "../../controllers/gmController.js";
import { Socket } from "socket.io";
import playerRepository from "../../repositories/playerRepository.js";

vi.mock("../../../services/statsService.js");

describe("gmController", () => {
    beforeEach(() => {
        connection.socket = null;

        console.log = vi.fn();
        console.error = vi.fn();
        console.warn = vi.fn();
    });

    afterEach(() => {
        vi.clearAllMocks();
    });

    describe("initConnection", () => {
        it("should establish a connection for the Game Master", () => {
            const socket = {
                id: "socket1",
                on: vi.fn(),
                emit: vi.fn(),
                disconnect: vi.fn(),
            } as unknown as Socket;

            initConnection(socket);

            expect(connection.socket).toBe(socket);

            expect(socket.on).toHaveBeenCalledWith(
                "disconnect",
                expect.any(Function)
            );
        });

        it("should refuse connection if one is already established", () => {
            const socket1 = {
                id: "socket1",
                on: vi.fn(),
                emit: vi.fn(),
                disconnect: vi.fn(),
            } as unknown as Socket;
            connection.socket = socket1;

            const socket2 = {
                id: "socket2",
                on: vi.fn(),
                emit: vi.fn(),
                disconnect: vi.fn(),
            } as unknown as Socket;

            initConnection(socket2);

            expect(connection.socket).toBe(socket1);

            expect(socket2.on).not.toHaveBeenCalled();
        });
    });

    describe("sendPlayerData", () => {
        beforeEach(() => {
            connection.socket = {
                id: "socket1",
                on: vi.fn(),
                emit: vi.fn(),
                disconnect: vi.fn(),
            } as unknown as Socket;
        });

        it("should send player data to the Game Master", () => {
            playerRepository.getAllPlayers = vi.fn().mockReturnValue([
                { name: "Player 1", stats: [] },
                { name: "Player 2", stats: [] },
            ]);

            sendPlayerData();

            expect(connection.socket!.emit).toHaveBeenCalledWith("players", [
                { name: "Player 1", stats: [] },
                { name: "Player 2", stats: [] },
            ]);
        });

        it("should not send player data if no connection exists", () => {
            connection.socket = null;

            sendPlayerData();

            expect(console.warn).toHaveBeenCalledWith(
                "No Game Master connected. Cannot send player data."
            );
        });
    });
});
