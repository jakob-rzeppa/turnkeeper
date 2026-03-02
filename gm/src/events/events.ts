/**
 * Game events matching the Rust GameEvent enum.
 */

export type GameEvent =
    | 'AddPlayer'
    | { ChangePlayerOrder: string[] }
    | { AttachUserToPlayer: { user_id: string; player_id: string } }
    | { DetachUserFromPlayer: { player_id: string } }
    | { Debug: string };
