import { useWsConnection } from '../api/useWsConnection';

/**
 * Game commands matching the Rust GameCommand enum.
 */
type GameCommand =
    | 'NextTurn'
    | 'PreviousTurn'
    | { SkipTurnToPlayer: { player_id: string } }
    | { SetNotes: string }
    | { SetHiddenNotes: string }
    | { AddPlayer: { player_id: string } }
    | {
          AddStatToPlayer: {
              player_id: string;
              stat_id: string;
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
    | { AddTradable: { tradable_id: string; name: string; initial_value: number } }
    | { RemoveTradable: { tradable_id: string } }
    | { ChangePlayerTradableValue: { player_id: string; tradable_id: string; new_value: number } }
    | { SendTradable: { from_id: string; to_id: string; tradable_id: string; amount: number } }
    | { AttachUserToPlayer: { user_id: string; player_id: string } }
    | { DetachUserFromPlayer: { player_id: string } }
    | { Debug: string };

export function useCommandEmitter() {
    const wsConnection = useWsConnection();

    const emit = (command: GameCommand) => {
        const payload = JSON.stringify(command);
        console.log('Emitting command:', payload);
        wsConnection.send(payload);
    };

    const nextTurn = () => emit('NextTurn');
    const previousTurn = () => emit('PreviousTurn');
    const skipTurnToPlayer = (player_id: string) => emit({ SkipTurnToPlayer: { player_id } });
    const setNotes = (notes: string) => emit({ SetNotes: notes });
    const setHiddenNotes = (notes: string) => emit({ SetHiddenNotes: notes });
    const addPlayer = () => {
        const player_id = crypto.randomUUID();
        emit({ AddPlayer: { player_id } });
    };
    const addStatToPlayer = (
        player_id: string,
        stat_key: string,
        stat_type: 'string' | 'number' | 'boolean',
        stat_value: string
    ) => {
        const stat_id = crypto.randomUUID();
        emit({ AddStatToPlayer: { player_id, stat_id, stat_key, stat_type, stat_value } });
    };
    const changeStatOfPlayer = (
        player_id: string,
        stat_id: string,
        stat_type: 'string' | 'number' | 'boolean',
        stat_value: string
    ) => emit({ ChangeStatOfPlayer: { player_id, stat_id, stat_type, stat_value } });
    const removeStatFromPlayer = (player_id: string, stat_id: string) =>
        emit({ RemoveStatFromPlayer: { player_id, stat_id } });
    const changePlayerOrder = (player_ids: string[]) => emit({ ChangePlayerOrder: player_ids });
    const addTradable = (name: string, initial_value: number) => {
        const tradable_id = crypto.randomUUID();
        emit({ AddTradable: { tradable_id, name, initial_value } });
    };
    const removeTradable = (tradable_id: string) => emit({ RemoveTradable: { tradable_id } });
    const changePlayerTradableValue = (player_id: string, tradable_id: string, new_value: number) =>
        emit({ ChangePlayerTradableValue: { player_id, tradable_id, new_value } });
    const sendTradable = (from_id: string, to_id: string, tradable_id: string, amount: number) =>
        emit({ SendTradable: { from_id, to_id, tradable_id, amount } });
    const attachUserToPlayer = (user_id: string, player_id: string) =>
        emit({ AttachUserToPlayer: { user_id, player_id } });
    const detachUserFromPlayer = (player_id: string) =>
        emit({ DetachUserFromPlayer: { player_id } });
    const debug = (message: string) => emit({ Debug: message });

    return {
        nextTurn,
        previousTurn,
        skipTurnToPlayer,
        setNotes,
        setHiddenNotes,
        addPlayer,
        addStatToPlayer,
        changeStatOfPlayer,
        removeStatFromPlayer,
        changePlayerOrder,
        addTradable,
        removeTradable,
        changePlayerTradableValue,
        sendTradable,
        attachUserToPlayer,
        detachUserFromPlayer,
        debug,
    };
}
