import playerRepository from "../repositories/playerRepository.js";
/**
 * The secret is a simple string that is generated when the player is created.
 * It is not meant to be secure, just to prevent accidental impersonation.
 *
 * @returns true if the secret is valid, false otherwise.
 */
export const isUserSecretValid = (
    playerId: string,
    providedSecret: string
): boolean => {
    const player = playerRepository.getPlayerById(playerId);

    if (!player) return false;

    const actualSecret = player.secret;
    return actualSecret == providedSecret;
};
