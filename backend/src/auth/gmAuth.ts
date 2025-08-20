import { Socket } from "socket.io";

let gmConnectionId: string | null = null;

export default function authenticateGm(socket: Socket) {
    socket.on("disconnect", () => {
        if (gmConnectionId === socket.id) {
            console.log(`Removing connection for Game Master: ${socket.id}`);
            gmConnectionId = null;
        }
    });

    if (gmConnectionId) {
        console.error(
            `Game Master connection refused to ${socket.id}: Game master already connected`
        );
        return false;
    }

    console.log(`Game Master connection established to ${socket.id}`);
    gmConnectionId = socket.id;
    return true;
}
