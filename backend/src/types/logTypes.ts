export interface LogEntry {
    details?: Record<string, unknown>;
    message: string;
    severity: LogSeverity;
    timestamp: Date;
}

type LogSeverity = "error" | "info" | "warning";
