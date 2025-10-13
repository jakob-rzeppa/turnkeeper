import { Server, Socket } from "socket.io";
import { afterEach, beforeAll, describe, expect, it, Mock, vi } from "vitest";
import {
    createUserSocket,
    handleDisconnect,
    onUserConnection,
} from "../../sockets/userSocket";
import { extractUserCredentials } from "../../util/extractUserCredentials";
import { authenticateUser } from "../../auth/userAuth";
import UserController from "../../connectionControllers/UserController";
import logger from "../../services/logger";

vi.mock("../../services/logger", () => ({
    default: {
        info: vi.fn(),
        warn: vi.fn(),
        error: vi.fn(),
    },
}));

vi.mock("../../connectionControllers/UserController", () => ({
    default: {
        registerSocket: vi.fn(),
        unregisterSocket: vi.fn(),
        getPlayerById: vi.fn(),
    },
}));

vi.mock("../../auth/userAuth", () => ({
    authenticateUser: vi.fn(),
}));

vi.mock("../../util/extractUserCredentials", () => ({
    extractUserCredentials: vi.fn(),
}));

describe("User Socket", () => {
    describe("createUserSocket", () => {
        it("should create User namespace and set up connection handler", () => {
            const mockServer: Server = {
                of: vi.fn().mockReturnValue({
                    on: vi.fn(),
                }),
            } as unknown as Server;

            createUserSocket(mockServer);

            expect(mockServer.of).toHaveBeenCalledWith("/user");
            expect(mockServer.of("/user").on).toHaveBeenCalledWith(
                "connection",
                expect.any(Function)
            );
        });
    });

    describe("onUserConnection", () => {
        let mockSocket: Socket;

        beforeAll(() => {
            mockSocket = {
                id: "mock-socket-id",
                emit: vi.fn(),
                disconnect: vi.fn(),
                on: vi.fn(),
            } as unknown as Socket;
        });

        afterEach(() => {
            vi.clearAllMocks();
        });

        it("should extract user credentials", () => {
            onUserConnection(mockSocket);

            expect(extractUserCredentials).toHaveBeenCalledWith(mockSocket);
        });

        it("should handle invalid credentials", () => {
            (extractUserCredentials as Mock).mockReturnValueOnce(null);

            onUserConnection(mockSocket);

            expect(logger.error).toHaveBeenCalledWith(
                expect.objectContaining({
                    message: "A user tried to connect but player was not found",
                    details: { credentials: null },
                })
            );
            expect(mockSocket.emit).toHaveBeenCalledWith("connection_error", {
                code: "INVALID_CREDENTIALS",
                message:
                    "Connection refused: Credentials do not match any player",
            });
            expect(mockSocket.disconnect).toHaveBeenCalled();
        });

        describe("when credentials are valid", () => {
            const mockCredentials = {
                playerId: "player-123",
                playerSecret: "secret-abc",
            };

            beforeAll(() => {
                (extractUserCredentials as Mock).mockReturnValue(
                    mockCredentials
                );
            });

            it("should call authenticateUser", () => {
                onUserConnection(mockSocket);

                expect(authenticateUser).toHaveBeenCalledWith(
                    mockSocket,
                    mockCredentials.playerId,
                    mockCredentials.playerSecret
                );
            });

            describe("when authentication succeeds", () => {
                beforeAll(() => {
                    (authenticateUser as Mock).mockReturnValue(true);
                });

                it("should log user connection", () => {
                    onUserConnection(mockSocket);

                    expect(logger.info).toHaveBeenCalledWith(
                        expect.objectContaining({
                            details: { playerId: mockCredentials.playerId },
                            message: "User connected",
                        })
                    );
                });

                it("should register the user socket", () => {
                    onUserConnection(mockSocket);

                    expect(UserController.registerSocket).toHaveBeenCalledWith(
                        mockCredentials.playerId,
                        mockSocket
                    );
                });

                it("should create disconnect handler", () => {
                    onUserConnection(mockSocket);

                    expect(mockSocket.on).toHaveBeenCalledWith(
                        "disconnect",
                        expect.any(Function)
                    );
                });
            });

            describe("when authentication fails", () => {
                beforeAll(() => {
                    (authenticateUser as Mock).mockReturnValue(false);
                });

                it("should not register the user socket", () => {
                    onUserConnection(mockSocket);

                    expect(
                        UserController.registerSocket
                    ).not.toHaveBeenCalled();
                });
            });
        });
    });

    describe("handleDisconnect", () => {
        it("should unregister the user socket and log disconnection", () => {
            const playerId = "player-123";

            handleDisconnect(playerId);

            expect(UserController.unregisterSocket).toHaveBeenCalledWith(
                playerId
            );
            expect(logger.info).toHaveBeenCalledWith(
                expect.objectContaining({
                    details: { playerId },
                    message: "User disconnected",
                })
            );
        });
    });
});
