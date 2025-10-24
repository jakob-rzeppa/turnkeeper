import { Socket } from "socket.io";
import { beforeEach, describe, expect, it, vi } from "vitest";

import UserGameEmitter from "../../../connectionEmitters/user/UserGameEmitter";
import gameStateHandler from "../../../services/gameStateHandler";

vi.mock("../../../services/gameStateHandler", () => ({
    default: {
        getGameState: vi.fn(),
    },
}));

describe("UserGameEmitter", () => {
    let mockSocket: Socket;
    let emitter: UserGameEmitter; // Only register one emitter at a time

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
                .spyOn(UserGameEmitter.prototype, "sendGameInfo")
                .mockReturnValue();

            emitter = new UserGameEmitter(mockSocket);

            expect(spy).toHaveBeenCalled();
            spy.mockRestore();
        });
    });

    describe("sendGameInfo", () => {
        it("should emit game:info with the correct payload", () => {
            vi.mocked(gameStateHandler.getGameState).mockReturnValue({
                currentPlayerIndex: 0,
                id: 1,
                playerOrder: [
                    { id: 1, name: "Player 1" },
                    { id: 2, name: "Player 2" },
                ],
                roundNumber: 3,
            });

            emitter = new UserGameEmitter(mockSocket);
            // The sendGameInfo is called in the constructor so it will be called two times, we clear the mocks to only test the second call
            vi.clearAllMocks();

            emitter.sendGameInfo();

            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit).toHaveBeenCalledWith("game:info", {
                gameState: {
                    currentPlayerIndex: 0,
                    id: 1,
                    playerOrder: [
                        { id: 1, name: "Player 1" },
                        { id: 2, name: "Player 2" },
                    ],
                    roundNumber: 3,
                },
            });
        });
    });
});
