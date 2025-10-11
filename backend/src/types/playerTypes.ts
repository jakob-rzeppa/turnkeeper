export interface Player {
    id: string;
    name: string;
    secret: string;
    stats: Stat[];
}

export interface Stat {
    name: string;
    value: boolean | number | string | string[];
}
