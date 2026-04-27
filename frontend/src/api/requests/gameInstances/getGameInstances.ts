import { errAsync, okAsync, ResultAsync } from 'neverthrow';
import type { GameInstanceMetadata } from '../../../types/gameInstances';
import { getWithAuth } from '../../httpApi';

interface GetGameInstancesResponse {
    game_instances: {
        id: string;
        name: string;
        game_id: string;
        player_count: number;
        current_round: number;
        gm_user_id: string;
        created_at: string;
        last_played_at: string;
    }[];
}

const isValidResponse = (res: unknown): res is GetGameInstancesResponse => {
    if (typeof res !== 'object' || res === null) return false;
    const g = res as { game_instances: Record<string, unknown>[] };
    return (
        Array.isArray(g.game_instances) &&
        g.game_instances.every(
            instance =>
                typeof instance.id === 'string' &&
                typeof instance.name === 'string' &&
                typeof instance.game_id === 'string' &&
                typeof instance.player_count === 'number' &&
                typeof instance.current_round === 'number' &&
                typeof instance.gm_user_id === 'string' &&
                typeof instance.created_at === 'string' &&
                typeof instance.last_played_at === 'string'
        )
    );
};

export const getGameInstances = (gameId: string): ResultAsync<GameInstanceMetadata[], string> => {
    return getWithAuth<GetGameInstancesResponse>(`/games/${gameId}/instances`).andThen(res => {
        if (!isValidResponse(res.data)) {
            return errAsync('Invalid response from server');
        }

        const instances: GameInstanceMetadata[] = res.data.game_instances.map(instance => ({
            ...instance,
            created_at: new Date(instance.created_at),
            last_played_at: new Date(instance.last_played_at),
        }));

        return okAsync(instances);
    });
};
