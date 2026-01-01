import { GmToBackendEventPayloads, UserToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';
import UserController from '../../connectionControllers/UserController.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('Gm Game Events Integration Tests', () => {
    let gmMockSocket: Socket;
    let userMockSocket: Socket;

    let gmEventHandlers: Record<string, (...args: unknown[]) => void>;
    let userEventHandlers: Record<string, (...args: unknown[]) => void>;

    const GM_EVENTS = {
        PLAYER_CREATE: 'players:create',
        MESSAGE_SEND: 'messages:send',
    } as const;

    const USER_EVENTS = {
        MESSAGE_SEND: 'messages:send',
    } as const;

    beforeAll(() => {
        gmEventHandlers = {};
        userEventHandlers = {};

        // Create a mock socket that captures gm event handlers
        gmMockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
            on: vi.fn((event: string, handler: (...args: unknown[]) => void) => {
                gmEventHandlers[event] = handler;
            }),
        } as unknown as Socket;

        GmController.registerSocket(gmMockSocket);

        // Create a mock socket that captures user event handlers
        userMockSocket = {
            emit: vi.fn(),
            id: 'mock-socket-id',
            on: vi.fn((event: string, handler: (...args: unknown[]) => void) => {
                userEventHandlers[event] = handler;
            }),
        } as unknown as Socket;

        UserController.registerSocket(1, userMockSocket);
    });

    beforeEach(() => {
        // Clear mocks before each test
        vi.clearAllMocks();
    });

    it('should register all messages event handlers', () => {
        // Verify that each event has a registered handler
        Object.values(GM_EVENTS).forEach((event) => {
            expect(gmEventHandlers[event]).toBeDefined();
        });

        Object.values(USER_EVENTS).forEach((event) => {
            expect(userEventHandlers[event]).toBeDefined();
        });
    });

    it('should create one players', () => {
        const payload: GmToBackendEventPayloads['players:create'] = {
            name: 'Player One',
        };

        const createHandler = gmEventHandlers[GM_EVENTS.PLAYER_CREATE] as (
            arg: GmToBackendEventPayloads['players:create'],
        ) => void;

        createHandler(payload);

        expect(gmMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(gmMockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({ id: 1, name: 'Player One' }),
                ]) as unknown[],
            }),
        );
    });

    it('should send one message from GM to Player', () => {
        const payload: GmToBackendEventPayloads['messages:send'] = {
            content: 'Hello Player!',
            playerId: 1,
        };

        const sendHandler = gmEventHandlers[GM_EVENTS.MESSAGE_SEND] as (
            arg: GmToBackendEventPayloads['messages:send'],
        ) => void;

        sendHandler(payload);

        expect(gmMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(gmMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Hello Player!', sendBy: 'gm' }),
            }),
        );
        expect(userMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(userMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Hello Player!', sendBy: 'gm' }),
            }),
        );
    });

    it('should send a second message from GM to user', () => {
        const payload: GmToBackendEventPayloads['messages:send'] = {
            content: 'Second Message!',
            playerId: 1,
        };

        const sendHandler = gmEventHandlers[GM_EVENTS.MESSAGE_SEND] as (
            arg: GmToBackendEventPayloads['messages:send'],
        ) => void;

        sendHandler(payload);

        expect(gmMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(gmMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Second Message!', sendBy: 'gm' }),
            }),
        );
        expect(userMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(userMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Second Message!', sendBy: 'gm' }),
            }),
        );
    });

    it('should send a message from Player to GM', () => {
        const payload: UserToBackendEventPayloads['messages:send'] = {
            content: 'Hello GM!',
        };

        const sendHandler = userEventHandlers[USER_EVENTS.MESSAGE_SEND] as (
            arg: UserToBackendEventPayloads['messages:send'],
        ) => void;

        sendHandler(payload);

        expect(userMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(userMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Hello GM!', sendBy: 'player' }),
            }),
        );
        expect(gmMockSocket.emit).toHaveBeenCalledTimes(1);
        expect(gmMockSocket.emit).toHaveBeenCalledWith(
            'messages:new',
            expect.objectContaining({
                message: expect.objectContaining({ content: 'Hello GM!', sendBy: 'player' }),
            }),
        );
    });
});
