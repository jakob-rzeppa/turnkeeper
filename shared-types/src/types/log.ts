export type LogSeverityType = "info" | "warning" | "error";

export interface LogEntryInterface {
    timestamp: Date;
    severity: LogSeverityType;
    message: string;
    details?: object;
}
