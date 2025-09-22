export type Stat = {
    name: string;
    value: boolean | number | string | string[];
};

export type Player = {
    id: string;
    name: string;
    secret: string;
    stats: Stat[];
};
