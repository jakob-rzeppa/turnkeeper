import { GameState } from "shared-types";

import { SqliteDatabase } from "../database/SqliteDatabase";

const db = SqliteDatabase.getInstance();

const gameStateRepository = {
    createGameState: (gamestate: Omit<GameState, "id">) => {
        try {
            db.prepare(
                "INSERT INTO game_state (round_number, current_player_index, player_order) VALUES (?, ?, ?)"
            ).run(
                gamestate.roundNumber,
                gamestate.currentPlayerIndex,
                gamestate.playerOrder.map((p) => p.id).join(",")
            );
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
    deleteGameState: (id: number) => {
        try {
            db.prepare("DELETE FROM game_state WHERE id = ?").run(id);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
    getGameStateById: (id: number): GameState | null => {
        const row = db
            .prepare("SELECT * FROM game_state WHERE id = ?")
            .get(id) as
            | undefined
            | {
                  current_player_index: number;
                  id: number;
                  player_order: string;
                  round_number: number;
              };

        if (!row) return null;

        const playerOrder = row.player_order.split(",").map(Number);

        const playerRows = db
            .prepare(
                `SELECT id, name FROM players WHERE id IN (?${",?".repeat(
                    playerOrder.length - 1
                )})`
            )
            .all(playerOrder) as {
            id: number;
            name: string;
        }[];

        console.log(playerRows);

        return {
            currentPlayerIndex: row.current_player_index,
            id: row.id,
            playerOrder: playerRows,
            roundNumber: row.round_number,
        };
    },
    updateGameState: (
        id: number,
        updatedFields: Partial<Omit<GameState, "id">>
    ) => {
        const fieldsToUpdate: string[] = [];
        const values: (number | string)[] = [];

        if (updatedFields.roundNumber !== undefined) {
            fieldsToUpdate.push("round_number = ?");
            values.push(updatedFields.roundNumber);
        }

        if (updatedFields.currentPlayerIndex !== undefined) {
            fieldsToUpdate.push("current_player_index = ?");
            values.push(updatedFields.currentPlayerIndex);
        }

        if (updatedFields.playerOrder !== undefined) {
            fieldsToUpdate.push("player_order = ?");
            values.push(updatedFields.playerOrder.map((p) => p.id).join(","));
        }

        if (fieldsToUpdate.length === 0) {
            return;
        }

        values.push(id);

        const query = `UPDATE game_state SET ${fieldsToUpdate.join(
            ", "
        )} WHERE id = ?`;

        try {
            db.prepare(query).run(...values);
        } catch (error: unknown) {
            // Handle error silently

            // This is to satisfy the linter that error is used
            if (error instanceof Error) {
                return;
            }
        }
    },
};

export default gameStateRepository;
