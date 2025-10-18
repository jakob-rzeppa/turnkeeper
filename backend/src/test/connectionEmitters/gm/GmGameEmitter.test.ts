import { Socket } from "socket.io";
import { beforeEach, describe, expect, it, vi } from "vitest";

import GmGameEmitter from "../../../connectionEmitters/gm/GmGameEmitter";
import playerRepository from "../../../repositories/playerRepository";
import { gameloop } from "../../../services/gameloop";

// Mock the player repository
vi.mock("../../../repositories/playerRepository", () => ({
    default: {
        getPlayerNameById: vi.fn(),
    },
}));

// Mock the gameloop service
vi.mock("../../../services/gameloop", () => ({
    gameloop: {
        getPlayerOrder: vi.fn(),
        getRoundInformation: vi.fn(),
        isInitialized: vi.fn(),
    },
}));

describe("GmGameEmitter", () => {
    let mockSocket: Socket;
    let emitter: GmGameEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: "mock-socket-id",
        } as unknown as Socket;
    });

    describe("constructor", () => {
        it("should call sendGameInfo on initialization", () => {
            const spy = vi
                .spyOn(GmGameEmitter.prototype, "sendGameInfo")
                .mockReturnValue();

            emitter = new GmGameEmitter(mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe("sendGameInfo", () => {
        it("should emit game:info with the correct payload", () => {
            vi.mocked(gameloop.getPlayerOrder).mockReturnValue([1, 2]);
            vi.mocked(gameloop.isInitialized).mockReturnValue(true);
            vi.mocked(gameloop.getRoundInformation).mockReturnValue({
                currentPlayerIndex: 0,
                roundNumber: 3,
            });
            vi.mocked(playerRepository.getPlayerNameById).mockImplementation(
                (id) => `Player ${String(id)}`
            );

            emitter = new GmGameEmitter(mockSocket);
            // The sendGameInfo is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            emitter.sendGameInfo();

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith("game:info", {
                isInitialized: true,
                playerOrder: [
                    { id: 1, name: "Player 1" },
                    { id: 2, name: "Player 2" },
                ],
                round: {
                    currentPlayerIndex: 0,
                    roundNumber: 3,
                },
            });
        });

        it("should emit game:info with the correct payload when getPlayerNameById does not find a player", () => {
            vi.mocked(gameloop.getPlayerOrder).mockReturnValue([1, 2]);
            vi.mocked(gameloop.isInitialized).mockReturnValue(true);
            vi.mocked(gameloop.getRoundInformation).mockReturnValue({
                currentPlayerIndex: 0,
                roundNumber: 3,
            });
            vi.mocked(playerRepository.getPlayerNameById).mockReturnValue(null);

            emitter = new GmGameEmitter(mockSocket);
            // The sendGameInfo is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            emitter.sendGameInfo();

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith("game:info", {
                isInitialized: true,
                playerOrder: [
                    { id: 1, name: "Player 1" },
                    { id: 2, name: "Player 2" },
                ],
                round: {
                    currentPlayerIndex: 0,
                    roundNumber: 3,
                },
            });
        });
    });
});
