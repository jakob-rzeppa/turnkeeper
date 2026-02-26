/**
 * Game events matching the Rust GameEvent enum.
 */

export type GameEvent = 'AddPlayer' | { ChangePlayerOrder: string[] } | { Debug: string };
