import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import gmController, { connection } from "../../controllers/gmController.js";
import { Socket } from "socket.io";
import statsService from "../../services/statsService.js";

vi.mock("../../../services/statsService.js");

describe("gmController", () => {
    beforeEach(() => {
        connection.socket = null;
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

            gmController.initConnection(socket);

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

            gmController.initConnection(socket2);

            expect(connection.socket).toBe(socket1);

            expect(socket2.on).not.toHaveBeenCalled();
        });
    });

    describe("sendStats", () => {
        it("should send stats to the Game Master", () => {
            const socket = {
                id: "socket1",
                on: vi.fn(),
                emit: vi.fn(),
                disconnect: vi.fn(),
            } as unknown as Socket;
            connection.socket = socket;

            statsService.getStatsForAllPlayers = vi.fn().mockReturnValue({
                player1: { score: 10 },
                player2: { score: 20 },
            });

            gmController.sendStats();

            expect(socket.emit).toHaveBeenCalledWith(
                "stats",
                expect.objectContaining({
                    player1: { score: 10 },
                    player2: { score: 20 },
                })
            );
        });

        it("should not send stats if no Game Master is connected", () => {
            gmController.sendStats();

            expect(connection.socket).toBeNull();
        });
    });
});
