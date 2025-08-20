export interface Player {
    name: string;
    isConnected: boolean;
    stats: Map<string, number | string | boolean | string[]>;
}