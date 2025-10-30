export interface Message {
    id: string;
    playerId: string | null;
    sendBy: "player" | "system" | "gm";
    content: string;
    timestamp: Date;
}
//# sourceMappingURL=message.d.ts.map