import { useWsConnection } from '../api/useWsConnection';
import type { GameEvent } from './events';

export function useEventEmitter() {
    const wsConnection = useWsConnection();

    const emit = (event: GameEvent) => {
        const payload = JSON.stringify(event);
        console.log('Emitting event:', payload);
        wsConnection.send(payload);
    };

    return { emit };
}
