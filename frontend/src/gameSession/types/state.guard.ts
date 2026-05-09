/*
 * Generated type guards for "state.ts".
 * WARNING: Do not manually change this file.
 */
import type { GameState } from "./state";

export function isGameState(obj: unknown): obj is GameState {
    const typedObj = obj as GameState
    return (
        (typedObj !== null &&
            typeof typedObj === "object" ||
            typeof typedObj === "function") &&
        typeof typedObj["round"] === "number" &&
        typeof typedObj["current_player_index"] === "number" &&
        Array.isArray(typedObj["game_stats"]) &&
        typedObj["game_stats"].every((e: any) =>
            (e !== null &&
                typeof e === "object" ||
                typeof e === "function") &&
            typeof e["name"] === "string" &&
            (e["value"] !== null &&
                typeof e["value"] === "object" ||
                typeof e["value"] === "function") &&
            (e["value"]["int_value"] === null ||
                typeof e["value"]["int_value"] === "number") &&
            (e["value"]["float_value"] === null ||
                typeof e["value"]["float_value"] === "number") &&
            (e["value"]["str_value"] === null ||
                typeof e["value"]["str_value"] === "string") &&
            (e["value"]["bool_value"] === null ||
                e["value"]["bool_value"] === false ||
                e["value"]["bool_value"] === true) &&
            (e["default"] !== null &&
                typeof e["default"] === "object" ||
                typeof e["default"] === "function") &&
            (e["default"]["int_value"] === null ||
                typeof e["default"]["int_value"] === "number") &&
            (e["default"]["float_value"] === null ||
                typeof e["default"]["float_value"] === "number") &&
            (e["default"]["str_value"] === null ||
                typeof e["default"]["str_value"] === "string") &&
            (e["default"]["bool_value"] === null ||
                e["default"]["bool_value"] === false ||
                e["default"]["bool_value"] === true) &&
            typeof e["visibility"] === "string"
        ) &&
        Array.isArray(typedObj["player_stats"]) &&
        typedObj["player_stats"].every((e: any) =>
            (e !== null &&
                typeof e === "object" ||
                typeof e === "function") &&
            typeof e["name"] === "string" &&
            Array.isArray(e["values"]) &&
            e["values"].every((e: any) =>
                Array.isArray(e) &&
                typeof e[0] === "string" &&
                (e[1] !== null &&
                    typeof e[1] === "object" ||
                    typeof e[1] === "function") &&
                (e[1]["int_value"] === null ||
                    typeof e[1]["int_value"] === "number") &&
                (e[1]["float_value"] === null ||
                    typeof e[1]["float_value"] === "number") &&
                (e[1]["str_value"] === null ||
                    typeof e[1]["str_value"] === "string") &&
                (e[1]["bool_value"] === null ||
                    e[1]["bool_value"] === false ||
                    e[1]["bool_value"] === true)
            ) &&
            (e["default"] !== null &&
                typeof e["default"] === "object" ||
                typeof e["default"] === "function") &&
            (e["default"]["int_value"] === null ||
                typeof e["default"]["int_value"] === "number") &&
            (e["default"]["float_value"] === null ||
                typeof e["default"]["float_value"] === "number") &&
            (e["default"]["str_value"] === null ||
                typeof e["default"]["str_value"] === "string") &&
            (e["default"]["bool_value"] === null ||
                e["default"]["bool_value"] === false ||
                e["default"]["bool_value"] === true) &&
            typeof e["visibility"] === "string"
        ) &&
        Array.isArray(typedObj["players"]) &&
        typedObj["players"].every((e: any) =>
            (e !== null &&
                typeof e === "object" ||
                typeof e === "function") &&
            typeof e["name"] === "string" &&
            (e["user_id"] === null ||
                typeof e["user_id"] === "string")
        )
    )
}
