export interface Message {
    id: number;
    playerId: number;
    content: string;
    timestamp: Date;
    sendBy: 'player' | 'gm';
}
