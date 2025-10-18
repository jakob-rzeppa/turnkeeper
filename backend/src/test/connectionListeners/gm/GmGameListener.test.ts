import { Socket } from "socket.io";
import { beforeEach, describe, expect, it, vi } from "vitest";

import GmGameListener from "../../../connectionListeners/gm/GmGameListener.js";
import { gameloop } from "../../../services/gameloop.js";

// Mock the gameloop service
vi.mock("../../../services/gameloop", () => ({
    gameloop: {
        end: vi.fn(),
        init: vi.fn(),
        nextTurn: vi.fn(),
        setPlayerOrder: vi.fn(),
    },
}));

describe("GmGameListener", () => {
    let mockSocket: Socket;
    let listener: GmGameListener;
    let eventHandlers: Record<string, Function>;

    beforeEach(() => {
        vi.clearAllMocks();
        eventHandlers = {};

        // Create a mock socket that captures event handlers
        mockSocket = {
            id: "mock-socket-id",
            on: vi.fn((event: string, handler: Function) => {
                eventHandlers[event] = handler;
            }),
        } as unknown as Socket;

        listener = new GmGameListener(mockSocket);
    });

    describe("constructor", () => {
        it("should register all game event listeners", () => {
            expect(mockSocket.on).toHaveBeenCalledWith(
                "game:init",
                expect.any(Function)
            );
            expect(mockSocket.on).toHaveBeenCalledWith(
                "game:turn:next",
                expect.any(Function)
            );
            expect(mockSocket.on).toHaveBeenCalledWith(
                "game:end",
                expect.any(Function)
            );
            expect(mockSocket.on).toHaveBeenCalledWith(
                "game:playerOrder:update",
                expect.any(Function)
            );
        });
    });

    describe("game:init event", () => {
        it("should initialize the gameloop with player IDs in order", () => {
            const playerIdsInOrder = [1, 3, 2, 5];

            eventHandlers["game:init"]({ playerIdsInOrder });

            expect(gameloop.init).toHaveBeenCalledWith(playerIdsInOrder);
            expect(gameloop.init).toHaveBeenCalledTimes(1);
        });
    });

    describe("game:turn:next event", () => {
        it("should call nextTurn on the gameloop", () => {
            eventHandlers["game:turn:next"]();

            expect(gameloop.nextTurn).toHaveBeenCalledTimes(1);
        });

        it("should not pass any arguments to nextTurn", () => {
            eventHandlers["game:turn:next"]();

            expect(gameloop.nextTurn).toHaveBeenCalledWith();
        });
    });

    describe("game:end event", () => {
        it("should call end on the gameloop", () => {
            eventHandlers["game:end"]();

            expect(gameloop.end).toHaveBeenCalledTimes(1);
        });

        it("should not pass any arguments to end", () => {
            eventHandlers["game:end"]();

            expect(gameloop.end).toHaveBeenCalledWith();
        });
    });

    describe("game:playerOrder:update event", () => {
        it("should update the player order in the gameloop", () => {
            const playerIdsInOrder = [5, 1, 3, 2];

            eventHandlers["game:playerOrder:update"]({ playerIdsInOrder });

            expect(gameloop.setPlayerOrder).toHaveBeenCalledWith(
                playerIdsInOrder
            );
            expect(gameloop.setPlayerOrder).toHaveBeenCalledTimes(1);
        });
    });
});
