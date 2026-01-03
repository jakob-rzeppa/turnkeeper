import { GmToBackendEventPayloads } from 'shared-types';
import { Socket } from 'socket.io';
import { beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController.js';

// Mock the config to use an in-memory database for testing
vi.mock('../../config/config.ts', () => ({
    default: {
        dbPath: ':memory:',
    },
}));

describe('Gm Game Events Integration Tests', () => {
    let mockSocket: Socket;
    let eventHandlers: Record<string, (...args: unknown[]) => void>;

    const EVENTS = {
        END: 'game:end',
        INIT: 'game:init',
        NEXT_TURN: 'game:turn:next',
        PLAYER_CREATE: 'players:create',
        PLAYER_DELETE: 'players:delete',
        UPDATE_HIDDEN_NOTES: 'game:hiddenNotes:update',
        UPDATE_NOTES: 'game:notes:update',
        UPDATE_PLAYER_ORDER: 'game:playerOrder:update',
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

    it('should register all game event handlers', () => {
        // Verify that each event has a registered handler
        Object.values(EVENTS).forEach((event) => {
            expect(eventHandlers[event]).toBeDefined();
        });
    });

    it('should create two players', () => {
        const payload1: GmToBackendEventPayloads['players:create'] = {
            name: 'Player One',
        };
        const payload2: GmToBackendEventPayloads['players:create'] = {
            name: 'Player Two',
        };

        const createHandler = eventHandlers[EVENTS.PLAYER_CREATE] as (
            arg: GmToBackendEventPayloads['players:create'],
        ) => void;

        createHandler(payload1);
        createHandler(payload2);

        expect(mockSocket.emit).toHaveBeenCalledTimes(2);
        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({ name: 'Player One' }),
                    expect.objectContaining({ name: 'Player Two' }),
                ]) as unknown[],
            }),
        );
    });

    it('should initialize the game', () => {
        const payload: GmToBackendEventPayloads['game:init'] = {
            playerIdsInOrder: [1, 2],
        };

        const initHandler = eventHandlers[EVENTS.INIT] as (
            arg: GmToBackendEventPayloads['game:init'],
        ) => void;

        initHandler(payload);

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 0,
                hiddenNotes: '',
                id: 1,
                notes: '',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should update game notes', () => {
        const notesPayload: GmToBackendEventPayloads['game:notes:update'] = {
            notes: 'These are the game notes.',
        };

        const notesHandler = eventHandlers[EVENTS.UPDATE_NOTES] as (
            arg: GmToBackendEventPayloads['game:notes:update'],
        ) => void;

        notesHandler(notesPayload);

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 0,
                hiddenNotes: '',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should update hidden notes', () => {
        const hiddenNotesPayload: GmToBackendEventPayloads['game:hiddenNotes:update'] = {
            hiddenNotes: 'These are the hidden notes.',
        };

        const hiddenNotesHandler = eventHandlers[EVENTS.UPDATE_HIDDEN_NOTES] as (
            arg: GmToBackendEventPayloads['game:hiddenNotes:update'],
        ) => void;

        hiddenNotesHandler(hiddenNotesPayload);

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 0,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should advance to the next turn', () => {
        const nextTurnHandler = eventHandlers[EVENTS.NEXT_TURN] as () => void;

        nextTurnHandler();

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 1,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should add a new player and add to player order', () => {
        const createPayload: GmToBackendEventPayloads['players:create'] = {
            name: 'Player Three',
        };

        const createHandler = eventHandlers[EVENTS.PLAYER_CREATE] as (
            arg: GmToBackendEventPayloads['players:create'],
        ) => void;

        createHandler(createPayload);

        expect(mockSocket.emit).toHaveBeenCalledWith(
            'players:info',
            expect.objectContaining({
                players: expect.arrayContaining([
                    expect.objectContaining({ name: 'Player Three' }),
                ]) as unknown[],
            }),
        );
        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 1,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                    { id: 3, name: 'Player Three' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should advance to the next turn again', () => {
        const nextTurnHandler = eventHandlers[EVENTS.NEXT_TURN] as () => void;

        nextTurnHandler();

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 2,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 2, name: 'Player Two' },
                    { id: 3, name: 'Player Three' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should delete a player and update player order', () => {
        const deletePayload: GmToBackendEventPayloads['players:delete'] = {
            playerId: 2,
        };

        const deleteHandler = eventHandlers[EVENTS.PLAYER_DELETE] as (
            arg: GmToBackendEventPayloads['players:delete'],
        ) => void;

        deleteHandler(deletePayload);

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 2,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 1, name: 'Player One' },
                    { id: 3, name: 'Player Three' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should update player order', () => {
        const updateOrderPayload: GmToBackendEventPayloads['game:playerOrder:update'] = {
            playerIdsInOrder: [3, 1],
        };

        const updateOrderHandler = eventHandlers[EVENTS.UPDATE_PLAYER_ORDER] as (
            arg: GmToBackendEventPayloads['game:playerOrder:update'],
        ) => void;

        updateOrderHandler(updateOrderPayload);

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: {
                currentPlayerIndex: 2,
                hiddenNotes: 'These are the hidden notes.',
                id: 1,
                notes: 'These are the game notes.',
                playerOrder: [
                    { id: 3, name: 'Player Three' },
                    { id: 1, name: 'Player One' },
                ],
                roundNumber: 1,
            },
        });
    });

    it('should end the game', () => {
        const endHandler = eventHandlers[EVENTS.END] as () => void;

        endHandler();

        expect(mockSocket.emit).toHaveBeenCalledWith('game:info', {
            gameState: null,
        });
    });

    it('should not fail when advancing turn with no game', () => {
        const nextTurnHandler = eventHandlers[EVENTS.NEXT_TURN] as () => void;

        nextTurnHandler();

        expect(mockSocket.emit).not.toHaveBeenCalledWith('game:info', expect.anything());
        expect(mockSocket.emit).toHaveBeenCalledWith('log:entry', {
            entry: {
                message: 'Failed to advance turn: Game state with ID 1 does not exist.',
                severity: 'error',
                timestamp: expect.any(Date) as Date,
            },
        });
    });
});
