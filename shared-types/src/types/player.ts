export interface Player {
    id: string;
    name: string;
    secret: string;
    stats: PlayerStat[];
}

export interface PlayerStat {
    name: string;
    value: boolean | number | string | string[];
}
