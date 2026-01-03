import { LogEntry } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmLogsEmitter from '../../../connectionEmitters/gm/GmLogsEmitter.js';

describe('GmLogsEmitter', () => {
    let mockSocket: Socket;
    let emitter: GmLogsEmitter; // Only register one emitter at a time

    beforeEach(() => {
        vi.clearAllMocks();

        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
        } as unknown as Socket;
    });

    describe('sendLogs', () => {
        it('should emit logs:info with the correct payload', () => {
            const log: LogEntry = {
                details: { key: 'value' },
                message: 'Log message',
                severity: 'info',
                timestamp: new Date(),
            };

            emitter = new GmLogsEmitter(mockSocket);
            emitter.sendLog(log);

            expect(mockSocket.emit).toHaveBeenCalledWith('log:entry', {
                entry: log,
            });
        });
    });
});
