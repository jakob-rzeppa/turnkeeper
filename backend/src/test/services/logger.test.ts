import { describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import logger from '../../services/logger.js';
import { formatLogEntry } from '../../util/formatLogEntry.js';

// Mock GmController to avoid side effects
vi.mock('../../connectionControllers/GmController', () => ({
    default: {
        getInstance: () => null,
    },
}));

vi.mock('../../util/formatLogEntry', () => ({
    formatLogEntry: vi.fn().mockReturnValue('formatted log entry'),
}));

vi.mock('../../connectionControllers/GmController', () => ({
    default: {
        getInstance: vi.fn().mockReturnValue({
            gmLogsEmitter: {
                sendLog: vi.fn(),
            },
        }),
    },
}));

describe('logger service', () => {
    it('should have error, info, warn, and log methods', () => {
        expect(typeof logger.error).toBe('function');
        expect(typeof logger.info).toBe('function');
        expect(typeof logger.warn).toBe('function');
        expect(typeof logger.log).toBe('function');
    });

    describe('error method', () => {
        it("should call log with severity 'error'", () => {
            const logSpy = vi.spyOn(logger, 'log');

            logger.error({ message: 'An error occurred' });

            expect(logSpy).toHaveBeenCalledWith(
                expect.objectContaining({
                    message: 'An error occurred',
                    severity: 'error',
                }),
            );

            logSpy.mockRestore();
        });
    });

    describe('warn method', () => {
        it("should call log with severity 'warn'", () => {
            const logSpy = vi.spyOn(logger, 'log');

            logger.warn({ message: 'A warning occurred' });

            expect(logSpy).toHaveBeenCalledWith(
                expect.objectContaining({
                    message: 'A warning occurred',
                    severity: 'warning',
                }),
            );

            logSpy.mockRestore();
        });
    });

    describe('info method', () => {
        it("should call log with severity 'info'", () => {
            const logSpy = vi.spyOn(logger, 'log');

            logger.info({ message: 'An info occurred' });

            expect(logSpy).toHaveBeenCalledWith(
                expect.objectContaining({
                    message: 'An info occurred',
                    severity: 'info',
                }),
            );

            logSpy.mockRestore();
        });
    });

    describe('log method', () => {
        it('should call formatLogEntry with the right parameters', () => {
            const logEntry = {
                message: 'A log occurred',
                severity: 'info' as const,
            };

            logger.log(logEntry);

            expect(formatLogEntry).toHaveBeenCalledWith(
                expect.objectContaining({
                    ...logEntry,
                    timestamp: expect.any(Date) as Date,
                }),
            );
        });

        it('should output to console based on severity', () => {
            const infoSpy = vi.spyOn(console, 'info');
            const errorSpy = vi.spyOn(console, 'error');
            const warnSpy = vi.spyOn(console, 'warn');

            logger.log({ message: 'Info log', severity: 'info' });
            expect(infoSpy).toHaveBeenCalledWith('formatted log entry');

            logger.log({ message: 'Error log', severity: 'error' });
            expect(errorSpy).toHaveBeenCalledWith('formatted log entry');

            logger.log({ message: 'Warning log', severity: 'warning' });
            expect(warnSpy).toHaveBeenCalledWith('formatted log entry');

            infoSpy.mockRestore();
            errorSpy.mockRestore();
            warnSpy.mockRestore();
        });

        it('should send logs to GmController if connected', () => {
            const logEntry = {
                message: 'A log for GM',
                severity: 'info' as const,
            };

            logger.log(logEntry);

            expect(GmController.getInstance).toHaveBeenCalled();
            expect(GmController.getInstance()?.gmLogsEmitter.sendLog).toHaveBeenCalledWith(
                expect.objectContaining({
                    ...logEntry,
                    timestamp: expect.any(Date) as Date,
                }),
            );
        });
    });
});
