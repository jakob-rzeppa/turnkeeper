export interface Player {
    id: number;
    name: string;
    secret: string;
    stats: PlayerStat[];
}

export interface PlayerStat {
    id: number;
    name: string;
    value: boolean | number | string | string[];
}
