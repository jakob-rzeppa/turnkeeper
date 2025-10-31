import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';

// Mock all emitters and listeners
vi.mock('../../connectionEmitters/gm/GmGameEmitter');
vi.mock('../../connectionEmitters/gm/GmLogsEmitter');
vi.mock('../../connectionEmitters/gm/GmPlayersEmitter');
vi.mock('../../connectionListeners/gm/GmGameListener');
vi.mock('../../connectionListeners/gm/GmPlayersListener');

describe('GmController', () => {
    let mockSocket: Socket;

    beforeEach(() => {
        // Clear all mocks before each test
        vi.clearAllMocks();

        // Reset the singleton instance
        GmController.unregisterSocket();

        // Create a mock socket
        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
            on: vi.fn(),
        } as unknown as Socket;
    });

    describe('Singleton Pattern', () => {
        it('should create only one instance', () => {
            GmController.registerSocket(mockSocket);
            const instance1 = GmController.getInstance();

            const mockSocket2 = {
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            GmController.registerSocket(mockSocket2);
            const instance2 = GmController.getInstance();

            // Both instances should be different objects (new instance replaces old)
            expect(instance1).not.toBe(instance2);
        });

        it('should return null when no socket is registered', () => {
            expect(GmController.getInstance()).toBeNull();
        });

        it('should return the instance after registration', () => {
            GmController.registerSocket(mockSocket);
            const instance = GmController.getInstance();

            expect(instance).not.toBeNull();
            expect(instance).toBeInstanceOf(GmController);
        });
    });

    describe('registerSocket', () => {
        it('should create emitter instances accessible via the controller', () => {
            GmController.registerSocket(mockSocket);
            const instance = GmController.getInstance();

            expect(instance).not.toBeNull();
            expect(instance?.gmGameEmitter).toBeDefined();
            expect(instance?.gmLogsEmitter).toBeDefined();
            expect(instance?.gmPlayersEmitter).toBeDefined();
        });

        it('should create listener instances accessible via the controller', () => {
            GmController.registerSocket(mockSocket);
            const instance = GmController.getInstance();

            expect(instance).not.toBeNull();
            expect(instance?.gmGameListener).toBeDefined();
            expect(instance?.gmPlayersListener).toBeDefined();
        });
    });

    describe('unregisterSocket', () => {
        it('should set the instance to null', () => {
            GmController.registerSocket(mockSocket);
            expect(GmController.getInstance()).not.toBeNull();

            GmController.unregisterSocket();
            expect(GmController.getInstance()).toBeNull();
        });

        it('should allow re-registration after unregistering', () => {
            GmController.registerSocket(mockSocket);
            const instance1 = GmController.getInstance();

            GmController.unregisterSocket();

            const mockSocket2 = {
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            GmController.registerSocket(mockSocket2);
            const instance2 = GmController.getInstance();

            expect(instance1).not.toBe(instance2);
            expect(instance2).not.toBeNull();
        });
    });

    describe('isConnected', () => {
        it('should return false when no socket is registered', () => {
            expect(GmController.isConnected()).toBe(false);
        });

        it('should return true when a socket is registered', () => {
            GmController.registerSocket(mockSocket);
            expect(GmController.isConnected()).toBe(true);
        });

        it('should return false after unregistering', () => {
            GmController.registerSocket(mockSocket);
            expect(GmController.isConnected()).toBe(true);

            GmController.unregisterSocket();
            expect(GmController.isConnected()).toBe(false);
        });
    });
});
