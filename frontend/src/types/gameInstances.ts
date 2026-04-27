export type GameInstanceMetadata = {
    id: string;
    name: string;

    game_id: string;

    player_count: number;
    current_round: number;

    gm_user_id: string;

    created_at: Date;
    last_played_at: Date;
};
