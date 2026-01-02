export interface Player {
    id: number;
    name: string;
    secret: string;
    notes: string;
    hiddenNotes: string;
    stats: PlayerStat[];
}

export interface PlayerStat {
    id: number;
    playerId: number;
    name: string;
    value: string | number | boolean;
}
