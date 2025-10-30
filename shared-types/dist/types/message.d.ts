export interface Message {
    id: number;
    playerId: number | null;
    sendBy: "player" | "system" | "gm";
    content: string;
    timestamp: Date;
}
//# sourceMappingURL=message.d.ts.map