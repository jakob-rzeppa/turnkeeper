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
    name: string;
    value: string | number | boolean;
}

// The UserPlayer type only exposes non-sensitive information and is used for player representation in the user interface.
export type UserPlayer = Pick<Player, "id" | "name" | "notes" | "stats">;
