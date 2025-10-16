import { Socket } from "socket.io";
import { afterEach, beforeAll, describe, expect, it, Mock, vi } from "vitest";

import { authenticateUser } from "../../auth/userAuth";
import UserController from "../../connectionControllers/UserController";
import playerRepository from "../../repositories/playerRepository";
import logger from "../../services/logger";

vi.mock("../../connectionControllers/UserController", () => ({
    default: {
        isConnected: vi.fn(),
    },
}));

vi.mock("../../services/logger", () => ({
    default: {
        error: vi.fn(),
        info: vi.fn(),
        warn: vi.fn(),
    },
}));

vi.mock("../../repositories/playerRepository", () => ({
    default: {
        getPlayerById: vi.fn(),
    },
}));

describe("userAuth", () => {
    afterEach(() => {
        vi.clearAllMocks();
    });

    describe("authenticateUser", () => {
        let mockSocket: Socket;

        beforeAll(() => {
            mockSocket = {
                disconnect: vi.fn(),
                emit: vi.fn(),
            } as unknown as Socket;
        });

        describe("when the player does not exist", () => {
            it("should log an error", () => {
                (playerRepository.getPlayerById as Mock).mockReturnValueOnce(
                    null
                );

                expect(authenticateUser(mockSocket, 2, "anySecret")).toBe(
                    false
                );

                expect(playerRepository.getPlayerById).toHaveBeenCalledWith(2);
                expect(logger.error).toHaveBeenCalledWith({
                    details: { playerId: 2 },
                    message:
                        "A user tried to connect but provided an invalid secret",
                });
                expect(mockSocket.emit).toHaveBeenCalledWith(
                    "connection_error",
                    expect.objectContaining({
                        code: "INVALID_SECRET",
                        message: "Connection refused: Invalid player secret",
                    })
                );
                expect(mockSocket.disconnect).toHaveBeenCalled();
            });
        });

        describe("when the secret is invalid", () => {
            it("should log an error, emit and connection_error and call socket.disconnect", () => {
                (playerRepository.getPlayerById as Mock).mockReturnValueOnce({
                    id: "player1",
                    secret: "actualSecret",
                });

                expect(authenticateUser(mockSocket, 1, "wrongSecret")).toBe(
                    false
                );

                expect(playerRepository.getPlayerById).toHaveBeenCalledWith(1);
                expect(logger.error).toHaveBeenCalledWith({
                    details: { playerId: 1 },
                    message:
                        "A user tried to connect but provided an invalid secret",
                });
                expect(mockSocket.emit).toHaveBeenCalledWith(
                    "connection_error",
                    expect.objectContaining({
                        code: "INVALID_SECRET",
                        message: "Connection refused: Invalid player secret",
                    })
                );
                expect(mockSocket.disconnect).toHaveBeenCalled();
            });
        });

        describe("when the player is already connected", () => {
            it("should log an error, emit and connection_error and call socket.disconnect", () => {
                (UserController.isConnected as Mock).mockReturnValueOnce(true);
                (playerRepository.getPlayerById as Mock).mockReturnValueOnce({
                    id: "player1",
                    secret: "actualSecret",
                });

                expect(authenticateUser(mockSocket, 1, "actualSecret")).toBe(
                    false
                );

                expect(UserController.isConnected).toHaveBeenCalledWith(1);
                expect(logger.error).toHaveBeenCalledWith({
                    details: { playerId: 1 },
                    message:
                        "A user tried to connect but another user is already connected for this player",
                });
                expect(mockSocket.emit).toHaveBeenCalledWith(
                    "connection_error",
                    expect.objectContaining({
                        code: "PLAYER_ALREADY_CONNECTED",
                        message:
                            "Connection refused: This player is already connected",
                    })
                );
                expect(mockSocket.disconnect).toHaveBeenCalled();
            });
        });

        describe("when the credentials are valid and a user for the player is not already connected", () => {
            it("should return true", () => {
                (UserController.isConnected as Mock).mockReturnValueOnce(false);
                (playerRepository.getPlayerById as Mock).mockReturnValueOnce({
                    id: "player1",
                    secret: "actualSecret",
                });

                expect(authenticateUser(mockSocket, 1, "actualSecret")).toBe(
                    true
                );

                expect(UserController.isConnected).toHaveBeenCalledWith(1);
                expect(logger.error).not.toHaveBeenCalled();
                expect(mockSocket.emit).not.toHaveBeenCalled();
                expect(mockSocket.disconnect).not.toHaveBeenCalled();
            });
        });
    });
});
