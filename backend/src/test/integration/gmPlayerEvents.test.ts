import { GmToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('Gm Player Integration Tests', () => {
    let mockSocket: Socket;
    let eventHandlers: Record<string, (...args: unknown[]) => void>;

    // Event names extracted from GmPlayersListener
    const PLAYER_EVENTS = {
        CREATE: 'players:create',
        DELETE: 'players:delete',
        STATS_CREATE: 'players:stats:create',
        STATS_REMOVE: 'players:stats:remove',
        STATS_UPDATE: 'players:stats:update',
        UPDATE: 'players:update',
    } as const;

    beforeAll(() => {
        eventHandlers = {};

        // Create a mock socket that captures event handlers
        mockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
            on: vi.fn((event: string, handler: (...args: unknown[]) => void) => {
                eventHandlers[event] = handler;
            }),
        } as unknown as Socket;

        GmController.registerSocket(mockSocket);
    });

    beforeEach(() => {
        // Clear mocks before each test
        vi.clearAllMocks();
    });

    it('should register all player event handlers', () => {
        // Verify that each event has a registered handler
        Object.values(PLAYER_EVENTS).forEach((event) => {
            expect(eventHandlers[event]).toBeDefined();
        });
    });

    it("should create a player on 'players:create' event", () => {
        const payload: GmToBackendEventPayloads['players:create'] = {
            name: 'First Player',
        };
        const createHandler = eventHandlers[PLAYER_EVENTS.CREATE] as (
            arg: GmToBackendEventPayloads['players:create'],
        ) => void;

        expect(createHandler).toBeDefined();

        createHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        name: 'First Player',
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should create a second player on 'players:create' event", () => {
        const payload: GmToBackendEventPayloads['players:create'] = {
            name: 'Second Player',
        };
        const createHandler = eventHandlers[PLAYER_EVENTS.CREATE] as (
            arg: GmToBackendEventPayloads['players:create'],
        ) => void;

        expect(createHandler).toBeDefined();

        createHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({ id: 2, name: 'Second Player' }),
                ]) as unknown[],
            }),
        );
    });

    it("should update a player on 'players:update' event", () => {
        const payload: GmToBackendEventPayloads['players:update'] = {
            playerData: { name: 'Updated First Player', notes: 'Some notes' },
            playerId: 1,
        };
        const updateHandler = eventHandlers[PLAYER_EVENTS.UPDATE] as (
            arg: GmToBackendEventPayloads['players:update'],
        ) => void;

        expect(updateHandler).toBeDefined();

        updateHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        name: 'Updated First Player',
                        notes: 'Some notes',
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should create a stat for all players on 'players:stats:create' event", () => {
        const payload: GmToBackendEventPayloads['players:stats:create'] = {
            scope: 'global',
            statData: { name: 'Score', value: '0' },
        };
        const statsCreateHandler = eventHandlers[PLAYER_EVENTS.STATS_CREATE] as (
            arg: GmToBackendEventPayloads['players:stats:create'],
        ) => void;

        expect(statsCreateHandler).toBeDefined();

        statsCreateHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        stats: expect.arrayContaining([
                            expect.objectContaining({
                                name: 'Score',
                                value: '0',
                            }),
                        ]) as unknown[],
                    }),
                    expect.objectContaining({
                        id: 2,
                        stats: expect.arrayContaining([
                            expect.objectContaining({
                                name: 'Score',
                                value: '0',
                            }),
                        ]) as unknown[],
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should create a stat for a player on 'players:stats:create' event", () => {
        const payload: GmToBackendEventPayloads['players:stats:create'] = {
            playerId: 1,
            scope: 'player',
            statData: { name: 'Health', value: '100' },
        };
        const statsCreateHandler = eventHandlers[PLAYER_EVENTS.STATS_CREATE] as (
            arg: GmToBackendEventPayloads['players:stats:create'],
        ) => void;

        expect(statsCreateHandler).toBeDefined();

        statsCreateHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        stats: expect.arrayContaining([
                            expect.objectContaining({
                                name: 'Health',
                                value: '100',
                            }),
                        ]) as unknown[],
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should update a player's stat on 'players:stats:update' event", () => {
        const payload: GmToBackendEventPayloads['players:stats:update'] = {
            playerId: 1,
            statId: 1,
            value: '1',
        };
        const statsUpdateHandler = eventHandlers[PLAYER_EVENTS.STATS_UPDATE] as (
            arg: GmToBackendEventPayloads['players:stats:update'],
        ) => void;

        expect(statsUpdateHandler).toBeDefined();

        statsUpdateHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        stats: expect.arrayContaining([
                            expect.objectContaining({
                                id: 1,
                                name: 'Score',
                                value: '1',
                            }),
                        ]) as unknown[],
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should delete a player's stat on 'players:stats:remove' event", () => {
        const payload: GmToBackendEventPayloads['players:stats:remove'] = {
            playerId: 1,
            statId: 2,
        };
        const statsRemoveHandler = eventHandlers[PLAYER_EVENTS.STATS_REMOVE] as (
            arg: GmToBackendEventPayloads['players:stats:remove'],
        ) => void;

        expect(statsRemoveHandler).toBeDefined();

        statsRemoveHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({
                        id: 1,
                        stats: expect.not.arrayContaining([
                            expect.objectContaining({ id: 2 }),
                        ]) as unknown[],
                    }),
                ]) as unknown[],
            }),
        );
    });

    it("should delete a player on 'players:delete' event", () => {
        const payload: GmToBackendEventPayloads['players:delete'] = {
            playerId: 2,
        };
        const deleteHandler = eventHandlers[PLAYER_EVENTS.DELETE] as (
            arg: GmToBackendEventPayloads['players:delete'],
        ) => void;

        expect(deleteHandler).toBeDefined();

        deleteHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.not.arrayContaining([
                    expect.objectContaining({ id: 2 }),
                ]) as unknown[],
            }),
        );
    });
});
