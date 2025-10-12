export type LogSeverity = "info" | "warning" | "error";

export interface LogEntry {
    timestamp: Date;
    severity: LogSeverity;
    message: string;
    details?: object;
}
