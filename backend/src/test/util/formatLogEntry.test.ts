import { describe, expect, it } from "vitest";
import { formatLogEntry } from "../../util/formatLogEntry";

describe("formatLogEntry", () => {
    it("should format log entries correctly", () => {
        const result = formatLogEntry({
            details: { userId: 123, action: "login" },
            message: "User logged in",
            severity: "info",
            timestamp: new Date("2024-01-01T12:00:00Z"),
        });

        const expected = `[2024-01-01T12:00:00.000Z] [INFO] User logged in | userId: 123, action: "login"`;

        expect(result).toBe(expected);
    });

    it("should format log entries without details correctly", () => {
        const result = formatLogEntry({
            message: "System started",
            severity: "info",
            timestamp: new Date("2024-01-01T00:00:00Z"),
        });

        const expected = `[2024-01-01T00:00:00.000Z] [INFO] System started`;

        expect(result).toBe(expected);
    });
});
