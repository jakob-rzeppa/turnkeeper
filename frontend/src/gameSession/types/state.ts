/** @see {isGameState} ts-auto-guard:type-guard */
export interface GameState {
    round: number;
    current_player_index: number;
    game_stats: GameStatState[];
    player_stats: PlayerStatState[];
    players: Player[];
}

export interface GameStatState {
    name: string;
    value: StatValue;
    default: StatValue;
    visibility: string;
}

export interface PlayerStatState {
    name: string;
    values: [string, StatValue][]; // Array of [player_name, value]
    default: StatValue;
    visibility: string;
}

export interface StatValue {
    int_value: number | null;
    float_value: number | null;
    str_value: string | null;
    bool_value: boolean | null;
}

export interface Player {
    name: string;
    user_id: string | null;
}
