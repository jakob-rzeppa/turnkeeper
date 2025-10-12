import { LogEntry } from "shared-types";

import GmController from "../connectionControllers/GmController.js";

function formatLogEntry(logEntry: LogEntry): string {
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

const logger = {
    error: (logEntry: Omit<LogEntry, "severity" | "timestamp">) => {
        logger.log({ ...logEntry, severity: "error" });
    },
    info: (logEntry: Omit<LogEntry, "severity" | "timestamp">) => {
        logger.log({ ...logEntry, severity: "info" });
    },
    // Log is a separate function to handle sending logs to different outputs in the future
    log: (logEntry: Omit<LogEntry, "timestamp">) => {
        const completeLogEntry: LogEntry = {
            ...logEntry,
            timestamp: new Date(),
        };

        const formattedLogEntry = formatLogEntry(completeLogEntry);
        switch (completeLogEntry.severity) {
            case "error":
                console.error(formattedLogEntry);
                break;
            case "info":
                console.info(formattedLogEntry);
                break;
            case "warning":
                console.warn(formattedLogEntry);
                break;
            default:
                console.log(formattedLogEntry);
        }

        // Send logs to the GM if connected
        GmController.getInstance()?.gmLogsEmitter.sendLog(completeLogEntry);
    },
    warn: (logEntry: Omit<LogEntry, "severity" | "timestamp">) => {
        logger.log({ ...logEntry, severity: "warning" });
    },
};

export default logger;
