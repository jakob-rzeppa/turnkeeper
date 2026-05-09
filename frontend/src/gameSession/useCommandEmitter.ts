import { useSession } from './useSession';

/**
 * Game commands matching the Rust GameCommand enum.
 */
type GameCommand =
    | 'Connect'
    | 'AdvanceTurn'
    | 'AddPlayer'
    | { ChangePlayerName: { player: string, new_name: string } }
    | { ChangePlayerOrder: { names_in_order: string[] } }
    | { AttachUserToPlayer: { player: string, user_id: string } }
    | { DetachUserFromPlayer: { player: string } }
    | { ExecuteAction: { action: string, params: Record<string, string> } } // Params: name -> value (int(5), float(3.14), string(hello), bool(true))
    | { Debug: string };

export function useCommandEmitter() {
    const session = useSession();

    const emit = (command: GameCommand) => {
        const payload = JSON.stringify(command);
        console.log('Emitting command:', payload);
        session.send(payload);
    };

    const advanceTurn = () => emit('AdvanceTurn');
    const addPlayer = () => emit('AddPlayer');
    const changePlayerName = (player: string, new_name: string) => emit({ ChangePlayerName: { player, new_name } });
    const changePlayerOrder = (names_in_order: string[]) => emit({ ChangePlayerOrder: { names_in_order } });
    const attachUserToPlayer = (player: string, user_id: string) => emit({ AttachUserToPlayer: { player, user_id } });
    const detachUserFromPlayer = (player: string) => emit({ DetachUserFromPlayer: { player } });
    const executeAction = (action: string, params: Record<string, string>) => emit({ ExecuteAction: { action, params } });
    const debug = (message: string) => emit({ Debug: message });

    return {
        advanceTurn,
        addPlayer,
        changePlayerName,
        changePlayerOrder,
        attachUserToPlayer,
        detachUserFromPlayer,
        executeAction,
        debug,
    };
}
