export interface Message {
    id: number;
    playerId: number;
    sendBy: "player" | "system" | "gm";
    content: string;
    timestamp: Date;
}
