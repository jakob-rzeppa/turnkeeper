import { Socket } from "socket.io";
import playerService from "../services/playerService.js";
import playerConnectionService from "../services/playerConnectionService.js";

export function authenticatePlayer(socket: Socket) {
    const name = socket.handshake.auth.name;

    // When refreshing the page the connection is lost therefore the client needs to be re-authenticated
    socket.on("disconnect", () => {
        if (playerConnectionService.getConnectionId(name) === socket.id) {
            console.log(`Removing connection for player: ${name}`);
            playerConnectionService.removeConnection(name);
        }
    });

    if (!name) {
        console.error(
            `Connection refused to ${socket.id}: Name is required for authentication`
        );
        return false;
    }

    if (playerConnectionService.checkIfPlayerAlreadyConnected(name)) {
        console.error(
            `Connection refused to ${socket.id}: User ${name} is already connected`
        );
        return false;
    }

    console.log(`Player connection for ${name} established: ${socket.id}`);
    playerService.addPlayer(name);
    playerConnectionService.setConnection(name, socket);
    return true;
}
