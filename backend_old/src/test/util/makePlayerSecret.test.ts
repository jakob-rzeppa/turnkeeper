import { describe, expect, it } from 'vitest';

import makePlayerSecret from '../../util/makePlayerSecret.js';

describe('makePlayerSecret', () => {
    it('generates a secret of the specified length', () => {
        const secret = makePlayerSecret({ length: 16 });
        expect(secret).toHaveLength(16);
    });

    it('only contains alphanumeric characters', () => {
        const secret = makePlayerSecret({ length: 16 });
        expect(secret).toMatch(/^[A-Za-z0-9]+$/);
    });
});
