/**
 * Game events matching the Rust GameEvent enum.
 */

export type GameEvent =
    | 'AddPlayer'
    | {
          AddStatToPlayer: {
              player_id: string;
              stat_key: string;
              stat_type: 'string' | 'number' | 'boolean';
              stat_value: string;
          };
      }
    | {
          ChangeStatOfPlayer: {
              player_id: string;
              stat_id: string;
              stat_type: 'string' | 'number' | 'boolean';
              stat_value: string;
          };
      }
    | { RemoveStatFromPlayer: { player_id: string; stat_id: string } }
    | { ChangePlayerOrder: string[] }
    | { AttachUserToPlayer: { user_id: string; player_id: string } }
    | { DetachUserFromPlayer: { player_id: string } }
    | { Debug: string };
