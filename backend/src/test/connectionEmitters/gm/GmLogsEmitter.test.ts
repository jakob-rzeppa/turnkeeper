import { Socket } from "socket.io";
import { beforeEach, describe, expect, it, vi } from "vitest";
import GmLogsEmitter from "../../../connectionEmitters/gm/GmLogsEmitter";
import { LogEntry } from "shared-types";

describe("GmLogsEmitter", () => {
    let mockSocket: Socket;
    let emitter: GmLogsEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: "mock-socket-id",
        } as unknown as Socket;
    });

    describe("sendLogs", () => {
        it("should emit logs:info with the correct payload", () => {
            const log: LogEntry = {
                timestamp: new Date(),
                severity: "info",
                message: "Log message",
                details: { key: "value" },
            };

            emitter = new GmLogsEmitter(mockSocket);
            emitter.sendLog(log);

            expect(mockSocket.emit).toHaveBeenCalledWith("log:entry", {
                entry: log,
            });
        });
    });
});
