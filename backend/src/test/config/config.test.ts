import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('dotenv', () => ({
    default: {
        config: vi.fn(),
    },
}));

// dynamic imports and vi.resetModules are used to ensure fresh module state for each test
describe('Config', () => {
    beforeEach(() => {
        vi.resetModules();
        vi.clearAllMocks();
    });

    describe('Config initialization', () => {
        it('should initialize with process.env values', async () => {
            process.env.DB_PATH = 'f';
            process.env.PORT = '200';

            const { default: config } = await import('../../config/config');

            expect(config.port).toBe(200);
            expect(config.dbPath).toBe('f');
        });

        it('should initialize with default values', async () => {
            delete process.env.DB_PATH;
            delete process.env.PORT;

            const { default: config } = await import('../../config/config');

            expect(config.port).toBe(3000);
            expect(config.dbPath).toBeNull();
        });
    });

    describe('Config structure', () => {
        it('should have exactly two properties', async () => {
            const { default: config } = await import('../../config/config');
            const keys = Object.keys(config);
            expect(keys).toHaveLength(2);
            expect(keys).toContain('dbPath');
            expect(keys).toContain('port');
        });

        it('should not allow modifications', async () => {
            const { default: config } = await import('../../config/config');
            const initialKeys = Object.keys(config);

            expect(() => (config.dbPath = 'test')).toThrowError(
                new TypeError("Cannot assign to read only property 'dbPath' of object '#<Object>'"),
            );
            expect(() => (config.port = 3000)).toThrowError(
                new TypeError("Cannot assign to read only property 'port' of object '#<Object>'"),
            );

            expect(Object.keys(config)).toHaveLength(initialKeys.length);
            expect(Object.keys(config)).toEqual(initialKeys);
        });
    });
});
