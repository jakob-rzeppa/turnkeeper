export interface Message {
    id: string;
    playerId: string | null; // null if the message is meant only for the GM
    sendBy: "player" | "system" | "gm";
    content: string;
    timestamp: Date;
}
