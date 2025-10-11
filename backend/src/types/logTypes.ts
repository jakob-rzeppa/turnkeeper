export interface LogEntry {
    details?: Record<string, any>;
    message: string;
    severity: LogSeverity;
    timestamp: Date;
}

type LogSeverity = "error" | "info" | "warning";
