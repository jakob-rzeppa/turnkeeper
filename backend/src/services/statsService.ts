export type PlayerStatValueTypes = string | number | boolean | string[];

export const stats = new Map<
    string, // playerName
    Map<string, PlayerStatValueTypes>
>();

export default {
    getStats: (playerName: string): Map<string, PlayerStatValueTypes> => {
        const playerStats = stats.get(playerName);

        if (!playerStats) return new Map<string, PlayerStatValueTypes>();

        // Return a deep copy of the player's stats
        const entries = playerStats.entries();

        const copiedEntries = entries.map(([key, value]) => {
            if (Array.isArray(value)) {
                return [key, [...value]] as [string, string[]];
            } else {
                return [key, value] as [string, PlayerStatValueTypes];
            }
        });

        return new Map(copiedEntries);
    },
    updateStat: (
        playerName: string,
        stat: string,
        value: PlayerStatValueTypes
    ) => {
        const playerStats = stats.get(playerName);
        if (!playerStats) {
            stats.set(playerName, new Map());
        }
        stats
            .get(playerName)!
            .set(stat, Array.isArray(value) ? [...value] : value);
    },
    addStat: (
        playerName: string,
        stat: string,
        value: PlayerStatValueTypes
    ) => {
        const playerStats = stats.get(playerName);
        if (!playerStats) {
            stats.set(playerName, new Map());
        }
        stats
            .get(playerName)!
            .set(stat, Array.isArray(value) ? [...value] : value);
    },
    removeStat: (playerName: string, stat: string) => {
        const playerStats = stats.get(playerName);
        if (playerStats) {
            playerStats.delete(stat);
        }
    },
};
