import { useSessionConnection } from './useSessionConnection';

/**
 * Game commands matching the Rust GameCommand enum.
 */
type GameCommand =
    | 'NextTurn'
    | { AddPlayer: { player_id: string } }
    | { ChangePlayerOrder: string[] }
    | { AttachUserToPlayer: { user_id: string; player_id: string } }
    | { DetachUserFromPlayer: { player_id: string } }
    | { Debug: string };

export function useCommandEmitter() {
    const connection = useSessionConnection();

    const emit = (command: GameCommand) => {
        const payload = JSON.stringify(command);
        console.log('Emitting command:', payload);
        connection.send(payload);
    };

    const nextTurn = () => emit('NextTurn');
    const addPlayer = () => {
        const player_id = crypto.randomUUID();
        emit({ AddPlayer: { player_id } });
    };
    const changePlayerOrder = (player_ids: string[]) => emit({ ChangePlayerOrder: player_ids });
    const attachUserToPlayer = (user_id: string, player_id: string) =>
        emit({ AttachUserToPlayer: { user_id, player_id } });
    const detachUserFromPlayer = (player_id: string) =>
        emit({ DetachUserFromPlayer: { player_id } });
    const debug = (message: string) => emit({ Debug: message });

    return {
        nextTurn,
        addPlayer,
        changePlayerOrder,
        attachUserToPlayer,
        detachUserFromPlayer,
        debug,
    };
}
