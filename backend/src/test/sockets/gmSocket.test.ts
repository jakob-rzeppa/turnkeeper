import {
    afterAll,
    afterEach,
    beforeAll,
    beforeEach,
    describe,
    expect,
    it,
    Mock,
    vi,
} from "vitest";
import { Server, Socket } from "socket.io";
import {
    createGmSocket,
    handleDisconnect,
    onGmConnection,
} from "../../sockets/gmSocket";
import GmController from "../../connectionControllers/GmController";
import logger from "../../services/logger";

vi.mock("../../services/logger", () => ({
    default: {
        info: vi.fn(),
        warn: vi.fn(),
        error: vi.fn(),
        debug: vi.fn(),
    },
}));

vi.mock("../../connectionControllers/GmController", () => ({
    default: {
        isConnected: vi.fn(),
        registerSocket: vi.fn(),
        unregisterSocket: vi.fn(),
    },
}));

describe("GM Socket", () => {
    describe("createGmSocket", () => {
        let mockServer: Server;

        beforeAll(() => {
            mockServer = {
                of: vi.fn().mockReturnValue({
                    on: vi.fn(),
                }),
            } as unknown as Server;
        });

        afterEach(() => {
            vi.clearAllMocks();
        });

        it("should create GM namespace and set up connection handler", () => {
            createGmSocket(mockServer);

            expect(mockServer.of).toHaveBeenCalledWith("/gm");
            expect(mockServer.of("/gm").on).toHaveBeenCalledWith(
                "connection",
                expect.any(Function)
            );
        });
    });

    describe("onGmConnection", () => {
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

        it("should register GmController", () => {
            onGmConnection(mockSocket);

            expect(GmController.registerSocket).toHaveBeenCalledWith(
                mockSocket
            );
        });

        it("should log GM connected", () => {
            onGmConnection(mockSocket);

            expect(logger.info).toHaveBeenCalledWith(
                expect.objectContaining({
                    details: { socketId: "mock-socket-id" },
                    message: "GM connected",
                })
            );
        });

        it("should create disconnect handler", () => {
            onGmConnection(mockSocket);

            expect(mockSocket.on).toHaveBeenCalledWith(
                "disconnect",
                expect.any(Function)
            );
        });

        describe("when a GM is already connected", () => {
            beforeAll(() => {
                (GmController.isConnected as Mock).mockReturnValue(true);
            });

            afterAll(() => {
                (GmController.isConnected as Mock).mockReturnValue(false);
            });

            it("should not register GmController", () => {
                onGmConnection(mockSocket);

                expect(GmController.registerSocket).not.toHaveBeenCalled();
            });

            it("should log a warning", () => {
                onGmConnection(mockSocket);

                expect(logger.warn).toHaveBeenCalledWith(
                    expect.objectContaining({
                        details: { socketId: "mock-socket-id" },
                        message:
                            "A GM tried to connect, but another GM is already connected",
                    })
                );
            });

            it("should emit connection_error event", () => {
                onGmConnection(mockSocket);

                expect(mockSocket.emit).toHaveBeenCalledWith(
                    "connection_error",
                    {
                        code: "GM_ALREADY_CONNECTED",
                        message:
                            "GM connection refused: Another GM is already connected",
                    }
                );
            });

            it("should disconnect the socket", () => {
                onGmConnection(mockSocket);

                expect(mockSocket.disconnect).toHaveBeenCalled();
            });
        });
    });

    describe("handleDisconnect", () => {
        let mockSocket: Socket;

        beforeAll(() => {
            mockSocket = {
                id: "mock-socket-id",
            } as unknown as Socket;
        });

        afterEach(() => {
            vi.clearAllMocks();
        });

        it("should unregister GmController", () => {
            handleDisconnect(mockSocket);

            expect(GmController.unregisterSocket).toHaveBeenCalled();
        });

        it("should log GM disconnected", () => {
            handleDisconnect(mockSocket);

            expect(logger.info).toHaveBeenCalledWith(
                expect.objectContaining({
                    details: { socketId: "mock-socket-id" },
                    message: "GM disconnected",
                })
            );
        });
    });
});
