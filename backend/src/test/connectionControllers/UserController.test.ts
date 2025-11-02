import { Socket } from 'socket.io';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import UserController from '../../connectionControllers/UserController.js';

// Mock all emitters
vi.mock('../../connectionEmitters/user/UserGameEmitter');
vi.mock('../../connectionEmitters/user/UserPlayersEmitter');

describe('UserController', () => {
    let mockSocket: Socket;
    const playerId = 1;

    beforeEach(() => {
        // Clear all mocks before each test
        vi.clearAllMocks();

        // Reset all instances
        const allInstances = UserController.getAllInstances();
        allInstances.forEach((instance) => {
            UserController.unregisterSocket(instance.playerId);
        });

        // Create a mock socket
        mockSocket = {
            disconnect: vi.fn(),
            emit: vi.fn(),
            id: 'mock-socket-id',
            on: vi.fn(),
        } as unknown as Socket;
    });

    describe('Multi-instance Pattern', () => {
        it('should create separate instances for different player IDs', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance1 = UserController.getInstance(playerId);

            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            const playerId2 = 2;
            UserController.registerSocket(playerId2, mockSocket2);
            const instance2 = UserController.getInstance(playerId2);

            // Both instances should be different objects
            expect(instance1).not.toBe(instance2);
            expect(instance1).toBeInstanceOf(UserController);
            expect(instance2).toBeInstanceOf(UserController);
        });

        it('should not create duplicate instances for the same player ID', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance1 = UserController.getInstance(playerId);

            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(playerId, mockSocket2);
            const instance2 = UserController.getInstance(playerId);

            // Should be the same instance (not replaced)
            expect(instance1).toBe(instance2);
        });

        it('should return undefined when no socket is registered for a player ID', () => {
            expect(UserController.getInstance(playerId)).toBeUndefined();
        });

        it('should return the instance after registration', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance = UserController.getInstance(playerId);

            expect(instance).toBeDefined();
            expect(instance).toBeInstanceOf(UserController);
        });
    });

    describe('getAllInstances', () => {
        it('should return an empty array when no instances exist', () => {
            const instances = UserController.getAllInstances();
            expect(instances).toEqual([]);
        });

        it('should return all registered instances', () => {
            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            const mockSocket3 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-3',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(1, mockSocket);
            UserController.registerSocket(2, mockSocket2);
            UserController.registerSocket(3, mockSocket3);

            const instances = UserController.getAllInstances();
            expect(instances).toHaveLength(3);
            expect(instances.every((instance) => instance instanceof UserController)).toBe(true);
        });
    });

    describe('registerSocket', () => {
        it('should create emitter instances accessible via the controller', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance = UserController.getInstance(playerId);

            expect(instance).toBeDefined();
            expect(instance?.userGameEmitter).toBeDefined();
            expect(instance?.userPlayersEmitter).toBeDefined();
        });

        it('should store the socket and playerId in the instance', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance = UserController.getInstance(playerId);

            expect(instance?.socket).toBe(mockSocket);
            expect(instance?.playerId).toBe(playerId);
        });
    });

    describe('unregisterSocket', () => {
        it('should remove the instance for the given player ID', () => {
            UserController.registerSocket(playerId, mockSocket);
            expect(UserController.getInstance(playerId)).toBeDefined();

            UserController.unregisterSocket(playerId);
            expect(UserController.getInstance(playerId)).toBeUndefined();
        });

        it('should allow re-registration after unregistering', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance1 = UserController.getInstance(playerId);

            UserController.unregisterSocket(playerId);

            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(playerId, mockSocket2);
            const instance2 = UserController.getInstance(playerId);

            expect(instance1).not.toBe(instance2);
            expect(instance2).toBeDefined();
        });

        it("should only remove the specified player's instance", () => {
            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(1, mockSocket);
            UserController.registerSocket(2, mockSocket2);

            UserController.unregisterSocket(1);

            expect(UserController.getInstance(1)).toBeUndefined();
            expect(UserController.getInstance(2)).toBeDefined();
        });
    });

    describe('isConnected', () => {
        it('should return false when no socket is registered for a player ID', () => {
            expect(UserController.isConnected(playerId)).toBe(false);
        });

        it('should return true when a socket is registered for a player ID', () => {
            UserController.registerSocket(playerId, mockSocket);
            expect(UserController.isConnected(playerId)).toBe(true);
        });

        it('should return false after unregistering a player ID', () => {
            UserController.registerSocket(playerId, mockSocket);
            expect(UserController.isConnected(playerId)).toBe(true);

            UserController.unregisterSocket(playerId);
            expect(UserController.isConnected(playerId)).toBe(false);
        });

        it('should return correct status for multiple player IDs', () => {
            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(1, mockSocket);
            UserController.registerSocket(2, mockSocket2);

            expect(UserController.isConnected(1)).toBe(true);
            expect(UserController.isConnected(2)).toBe(true);
            expect(UserController.isConnected(3)).toBe(false);
        });
    });

    describe('disconnect', () => {
        it('should call socket.disconnect() on the instance', () => {
            UserController.registerSocket(playerId, mockSocket);
            const instance = UserController.getInstance(playerId);

            instance?.disconnect();

            expect(mockSocket.disconnect).toHaveBeenCalledOnce();
        });

        it('should disconnect the correct socket for each instance', () => {
            const mockSocket2 = {
                disconnect: vi.fn(),
                emit: vi.fn(),
                id: 'mock-socket-id-2',
                on: vi.fn(),
            } as unknown as Socket;

            UserController.registerSocket(1, mockSocket);
            UserController.registerSocket(2, mockSocket2);

            const instance1 = UserController.getInstance(1);
            instance1?.disconnect();

            expect(mockSocket.disconnect).toHaveBeenCalledOnce();
            expect(mockSocket2.disconnect).not.toHaveBeenCalled();
        });
    });
});
