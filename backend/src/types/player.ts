export interface Player {
    name: string;
    currentConnectionId: string | null;
    stats: Map<string, number | string | boolean | string[]>;
}
