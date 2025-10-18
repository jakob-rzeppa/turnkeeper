import { LogEntry } from "shared-types";

export function formatLogEntry(logEntry: LogEntry): string {
    const { details, message, severity, timestamp } = logEntry;
    let formattedEntry = `[${timestamp.toISOString()}] [${severity.toUpperCase()}] ${message}`;

    if (details) {
        const detailsStr = Object.entries(details)
            .map(([key, value]) => `${key}: ${JSON.stringify(value)}`)
            .join(", ");
        formattedEntry += ` | ${detailsStr}`;
    }

    return formattedEntry;
}
