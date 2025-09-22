import GmLogsHandler from "../connectionHandlers/gm/gmLogsHandler.js";
import { LogEntry } from "../types/logTypes.js";

function formatLogEntry(logEntry: LogEntry): string {
    const { timestamp, severity, message, data } = logEntry;
    let formattedEntry = `[${timestamp.toISOString()}] [${severity.toUpperCase()}] ${message}`;

    if (data) {
        const dataStr = Object.entries(data)
            .map(([key, value]) => `${key}: ${JSON.stringify(value)}`)
            .join(", ");
        formattedEntry += ` | ${dataStr}`;
    }

    return formattedEntry;
}

const logger = {
    info: (logEntry: Omit<LogEntry, "timestamp" | "severity">) => {
        logger.log({ ...logEntry, severity: "info" });
    },
    warning: (logEntry: Omit<LogEntry, "timestamp" | "severity">) => {
        logger.log({ ...logEntry, severity: "warning" });
    },
    error: (logEntry: Omit<LogEntry, "timestamp" | "severity">) => {
        logger.log({ ...logEntry, severity: "error" });
    },
    // Log is a separate function to handle sending logs to different outputs in the future
    log: (logEntry: Omit<LogEntry, "timestamp">) => {
        const completeLogEntry: LogEntry = {
            ...logEntry,
            timestamp: new Date(),
        };

        const formattedLogEntry = formatLogEntry(completeLogEntry);
        switch (completeLogEntry.severity) {
            case "info":
                console.info(formattedLogEntry);
                break;
            case "warning":
                console.warn(formattedLogEntry);
                break;
            case "error":
                console.error(formattedLogEntry);
                break;
            default:
                console.log(formattedLogEntry);
        }

        // Send logs to the GM if connected
        const gmLogsHandler = GmLogsHandler.getInstance();
        if (gmLogsHandler) {
            gmLogsHandler.sendLog(completeLogEntry);
        }
    },
};

export default logger;
