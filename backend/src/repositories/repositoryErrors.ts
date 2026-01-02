// --- NotFound ---

export class NotFound extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'NotFound';
    }
}

// --- Conflict ---

export class Conflict extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'Conflict';
    }
}

// --- ValidationError ---

export class ValidationError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'ValidationError';
    }
}

// --- DatabaseError ---

export class DatabaseError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'DatabaseError';
    }
}
