/** @see {isDisplayTemplate} ts-auto-guard:type-guard */
export interface DisplayTemplate {
    stats: GameStat[];
    player_stats: PlayerStat[];
    actions: Action[];
}

export interface GameStat {
    name: string;
    datatype: 'string' | 'int' | 'float' | 'boolean';
    default: string;
    visibility: 'public' | 'private';
    pos: string;
}

export interface PlayerStat {
    name: string;
    datatype: 'string' | 'int' | 'float' | 'boolean';
    default: string;
    visibility: 'public' | 'protected' | 'private';
    pos: string;
}

export interface Action {
    name: string;
    parameters: string[];
    execution_triggers: string[];
    visibility: 'public' | 'private';
    source_code: string;
    pos: string;
}
