export interface PlayerInterface {
    id: string;
    name: string;
    secret: string;
    stats: StatInterface[];
}
export interface StatInterface {
    name: string;
    value: boolean | number | string | string[];
}
//# sourceMappingURL=player.d.ts.map