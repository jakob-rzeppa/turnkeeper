import playerRepository from "../repositories/playerRepository.js";

export const isUserSecretValid = (
    playerId: string,
    providedSecret: string
): boolean => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player) return false;

    const actualSecret = player.secret;
    return actualSecret == providedSecret;
};
