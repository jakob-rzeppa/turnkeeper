export interface Player {
    id: number;
    name: string;
    secret: string;
    notes: string;
    stats: PlayerStat[];
}
export interface PlayerStat {
    id: number;
    name: string;
    value: string | number | boolean;
}
export type UserPlayer = Pick<Player, "id" | "name" | "notes" | "stats">;
//# sourceMappingURL=player.d.ts.map