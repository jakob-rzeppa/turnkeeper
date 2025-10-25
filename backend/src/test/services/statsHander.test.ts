import { beforeEach, describe, expect, it, vi } from 'vitest';

import GmController from '../../connectionControllers/GmController';
import UserController from '../../connectionControllers/UserController';
import { statsRepository } from '../../repositories/statsRepository';
import { statsHandler } from '../../services/statsHandler';

vi.mock('../../repositories/statsRepository.ts', () => ({
    statsRepository: {
        createStatForAllPlayers: vi.fn(),
        createStatForPlayer: vi.fn(),
        removeStatFromPlayer: vi.fn(),
        updateStatForPlayer: vi.fn(),
    },
}));

vi.mock('../../connectionControllers/GmController.ts', () => ({
    default: {
        getInstance: vi.fn().mockReturnValue({
            gmPlayersEmitter: {
                sendPlayers: vi.fn(),
            },
        }),
    },
}));

vi.mock('../../connectionControllers/UserController.ts', () => ({
    default: {
        getAllInstances: vi.fn().mockReturnValue([
            {
                userPlayersEmitter: {
                    sendOwnPlayer: vi.fn(),
                },
            },
        ]),
        getInstance: vi.fn().mockReturnValue({
            userPlayersEmitter: {
                sendOwnPlayer: vi.fn(),
            },
        }),
    },
}));

describe('statsHandler', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe('createStatForAllPlayers', () => {
        it('should create a stat for all players', () => {
            const statData = { name: 'health', playerId: 0, value: '100' };

            statsHandler.createStatForAllPlayers(statData);

            expect(statsRepository.createStatForAllPlayers).toHaveBeenCalledWith(statData);
        });

        it('should notify GM and all users after creating stat', () => {
            const statData = { name: 'health', playerId: 0, value: '100' };

            statsHandler.createStatForAllPlayers(statData);

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(UserController.getAllInstances).toHaveBeenCalled();
            UserController.getAllInstances().forEach((instance) => {
                expect(instance.userPlayersEmitter.sendOwnPlayer).toHaveBeenCalled();
            });
        });

        it('should allow statData with empty value', () => {
            const statData = { name: 'stamina', playerId: 0, value: '' };

            statsHandler.createStatForAllPlayers(statData);

            expect(statsRepository.createStatForAllPlayers).toHaveBeenCalledWith(statData);
        });
    });

    describe('createStatForPlayer', () => {
        it('should create a stat for a specific player', () => {
            const playerId = 1;
            const statData = { name: 'mana', playerId: 0, value: '50' };

            statsHandler.createStatForPlayer({ playerId, statData });

            expect(statsRepository.createStatForPlayer).toHaveBeenCalledWith(playerId, statData);
        });

        it('should notify GM and the specific user after creating stat', () => {
            const playerId = 1;
            const statData = { name: 'mana', playerId: 0, value: '50' };

            statsHandler.createStatForPlayer({ playerId, statData });

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer,
            ).toHaveBeenCalled();
        });

        it('should allow statData with empty value', () => {
            const statData = { name: 'stamina', playerId: 0, value: '' };

            statsHandler.createStatForPlayer({ playerId: 0, statData });

            expect(statsRepository.createStatForPlayer).toHaveBeenCalledWith(0, statData);
        });
    });

    describe('updateStatValue', () => {
        it('should update the value of a specific stat for a player', () => {
            const playerId = 1;
            const statId = 2;
            const newData = { value: '75' };

            statsHandler.updateStat({ newData, playerId, statId });

            expect(statsRepository.updateStatForPlayer).toHaveBeenCalledWith(
                playerId,
                statId,
                newData,
            );
        });

        it('should notify GM and the specific user after updating stat', () => {
            const playerId = 1;
            const statId = 2;
            const newData = { value: '75' };

            statsHandler.updateStat({ newData, playerId, statId });

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer,
            ).toHaveBeenCalled();
        });

        it('should allow updating stat to an empty value', () => {
            const playerId = 1;
            const statId = 2;
            const newData = { value: '' };

            statsHandler.updateStat({ newData, playerId, statId });

            expect(statsRepository.updateStatForPlayer).toHaveBeenCalledWith(
                playerId,
                statId,
                newData,
            );
        });

        it('should allow updating name and value of a stat simultaneously', () => {
            const playerId = 1;
            const statId = 2;
            const newData = { name: 'energy', value: '90' };

            statsHandler.updateStat({ newData, playerId, statId });

            expect(statsRepository.updateStatForPlayer).toHaveBeenCalledWith(
                playerId,
                statId,
                newData,
            );
        });
    });

    describe('removeStat', () => {
        it('should remove a specific stat from a player', () => {
            const playerId = 1;
            const statId = 2;

            statsHandler.removeStat({ playerId, statId });

            expect(statsRepository.removeStatFromPlayer).toHaveBeenCalledWith(playerId, statId);
        });

        it('should notify GM and the specific user after removing stat', () => {
            const playerId = 1;
            const statId = 2;

            statsHandler.removeStat({ playerId, statId });

            expect(GmController.getInstance()?.gmPlayersEmitter.sendPlayers).toHaveBeenCalled();
            expect(
                UserController.getInstance(playerId)?.userPlayersEmitter.sendOwnPlayer,
            ).toHaveBeenCalled();
        });
    });
});
