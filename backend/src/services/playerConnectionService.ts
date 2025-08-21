import { Socket } from "socket.io";

// player name -> socket
export const connections = new Map<string, Socket | null>();

export default {
    getConnectionId: (name: string): string | null => {
        const socket = connections.get(name);
        return socket ? socket.id : null;
    },
    checkIfPlayerAlreadyConnected: (name: string): boolean => {
        const socket = connections.get(name);
        return socket !== undefined ? socket !== null : false;
    },
    setConnection: (name: string, socket: Socket): void => {
        connections.set(name, socket);
    },
    removeConnection: (name: string) => {
        connections.set(name, null);
    },
};
