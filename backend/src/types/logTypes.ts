type LogSeverity = "info" | "warning" | "error";

export type LogEntry = {
    timestamp: Date;
    severity: LogSeverity;
    message: string;
    data?: { [key: string]: any };
};
