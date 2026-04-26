use sqlx::{Acquire, Row, SqlitePool};

use crate::{
    application::{
        common::error::DatabaseError, game_instance::contracts::GameInstanceRepositoryContract,
    },
    domain::common::identifier::Identifier,
    domain::game::{
        entities::game_instance::GameInstance,
        projections::game_instance_metadata::GameInstanceMetadataProjection,
    },
};

pub struct SqliteGameInstanceRepository {
    db: SqlitePool,
}

impl SqliteGameInstanceRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameInstanceRepositoryContract for SqliteGameInstanceRepository {
    async fn get_by_id(&self, id: Identifier) -> Result<Option<GameInstance>, DatabaseError> {
        let mut conn =
            self.db.acquire().await.map_err(|e| {
                DatabaseError::Custom(format!("Failed to acquire connection: {}", e))
            })?;

        let id_str = id.to_string();

        // Load the game instance
        let row = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                current_player_index,
                round,
                gm_user_id,
                source_game,
                created_at,
                last_played_at
            FROM game_instances 
            WHERE id = ?
            "#,
        )
        .bind(&id_str)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DatabaseError::Custom(format!("Failed to fetch game instance: {}", e)))?;

        match row {
            Some(row) => {
                let instance_id: String = row.get("id");
                let name: String = row.get("name");
                let current_player_index: i64 = row.get("current_player_index");
                let round: i64 = row.get("round");
                let gm_user_id_str: String = row.get("gm_user_id");
                let source_game_id_str: String = row.get("source_game");
                let created_at_str: String = row.get("created_at");
                let last_played_at_str: String = row.get("last_played_at");

                // Parse basic types
                let instance_id_parsed = Identifier::parse_str(&instance_id).map_err(|e| {
                    DatabaseError::Custom(format!("Failed to parse instance id: {}", e))
                })?;
                let gm_user_id = Identifier::parse_str(&gm_user_id_str).map_err(|e| {
                    DatabaseError::Custom(format!("Failed to parse gm_user_id: {}", e))
                })?;
                let created_at =
                    crate::domain::common::date_time::DateTime::parse_str(&created_at_str)
                        .map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse created_at: {}", e))
                        })?;
                let last_played_at =
                    crate::domain::common::date_time::DateTime::parse_str(&last_played_at_str)
                        .map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse last_played_at: {}", e))
                        })?;

                // Load source game
                let source_game_row = sqlx::query(
                    r#"
                    SELECT id, name, description, source_code, created_at, updated_at
                    FROM games WHERE id = ?
                    "#,
                )
                .bind(&source_game_id_str)
                .fetch_optional(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch source game: {}", e)))?
                .ok_or_else(|| DatabaseError::Custom("Source game not found".to_string()))?;

                let game_id: String = source_game_row.get("id");
                let game_name: String = source_game_row.get("name");
                let game_description: String = source_game_row.get("description");
                let game_source_code: String = source_game_row.get("source_code");
                let game_created_at: String = source_game_row.get("created_at");
                let game_updated_at: String = source_game_row.get("updated_at");

                let game_id_parsed = Identifier::parse_str(&game_id).map_err(|e| {
                    DatabaseError::Custom(format!("Failed to parse game id: {}", e))
                })?;
                let game_created_at_parsed =
                    crate::domain::common::date_time::DateTime::parse_str(&game_created_at)
                        .map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse game created_at: {}", e))
                        })?;
                let game_updated_at_parsed =
                    crate::domain::common::date_time::DateTime::parse_str(&game_updated_at)
                        .map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse game updated_at: {}", e))
                        })?;

                let source_game = crate::domain::game::entities::game::Game::new_raw(
                    game_id_parsed,
                    game_name,
                    game_description,
                    game_source_code,
                    game_created_at_parsed,
                    game_updated_at_parsed,
                );

                // Load game stats
                let game_stats_rows = sqlx::query(
                    r#"
                    SELECT id, name, int_value, float_value, string_value, bool_value,
                           default_int_value, default_float_value, default_string_value, default_bool_value,
                           visibility, pos
                    FROM game_stats WHERE game_instance_id = ?
                    "#
                )
                .bind(&instance_id)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch game stats: {}", e)))?;

                let mut game_stats = Vec::new();
                for stat_row in game_stats_rows {
                    let stat_id: String = stat_row.get("id");
                    let stat_name: String = stat_row.get("name");
                    let int_value: Option<i64> = stat_row.get("int_value");
                    let float_value: Option<f64> = stat_row.get("float_value");
                    let string_value: Option<String> = stat_row.get("string_value");
                    let bool_value: Option<bool> = stat_row.get("bool_value");
                    let default_int_value: Option<i64> = stat_row.get("default_int_value");
                    let default_float_value: Option<f64> = stat_row.get("default_float_value");
                    let default_string_value: Option<String> = stat_row.get("default_string_value");
                    let default_bool_value: Option<bool> = stat_row.get("default_bool_value");
                    let visibility_str: String = stat_row.get("visibility");

                    let stat_id_parsed = Identifier::parse_str(&stat_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse stat id: {}", e))
                    })?;

                    // Reconstruct StatValue from columns
                    let value = if let Some(v) = int_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Int(v)
                    } else if let Some(v) = float_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Float(v)
                    } else if let Some(v) = string_value {
                        crate::domain::game::value_objects::stat_value::StatValue::String(v)
                    } else if let Some(v) = bool_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Bool(v)
                    } else {
                        return Err(DatabaseError::Custom("Game stat has no value".to_string()));
                    };

                    let default_value = if let Some(v) = default_int_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Int(v)
                    } else if let Some(v) = default_float_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Float(v)
                    } else if let Some(v) = default_string_value {
                        crate::domain::game::value_objects::stat_value::StatValue::String(v)
                    } else if let Some(v) = default_bool_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Bool(v)
                    } else {
                        return Err(DatabaseError::Custom(
                            "Game stat has no default value".to_string(),
                        ));
                    };

                    let visibility = match visibility_str.as_str() {
                        "Public" => crate::domain::game::value_objects::stat_visibility::GameStatVisibility::Public,
                        "Private" => crate::domain::game::value_objects::stat_visibility::GameStatVisibility::Private,
                        _ => return Err(DatabaseError::Custom(format!("Invalid visibility: {}", visibility_str))),
                    };

                    let pos_str: String = stat_row.get("pos");
                    let pos = crate::domain::common::position::Position::from_str(&pos_str)
                        .ok_or_else(|| DatabaseError::Custom(format!("Failed to parse position: {}", pos_str)))?;

                    game_stats.push(crate::domain::game::entities::stat::GameStat::new_raw(
                        stat_id_parsed,
                        stat_name,
                        value,
                        default_value,
                        visibility,
                        pos,
                    ));
                }

                // Load player stats with their values
                let player_stats_rows = sqlx::query(
                    r#"
                    SELECT id, name, 
                           default_int_value, default_float_value, default_string_value, default_bool_value,
                           visibility, pos
                    FROM player_stats WHERE game_instance_id = ?
                    "#
                )
                .bind(&instance_id)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch player stats: {}", e)))?;

                let mut player_stats = Vec::new();
                for pstat_row in player_stats_rows {
                    let pstat_id: String = pstat_row.get("id");
                    let pstat_name: String = pstat_row.get("name");
                    let default_int_value: Option<i64> = pstat_row.get("default_int_value");
                    let default_float_value: Option<f64> = pstat_row.get("default_float_value");
                    let default_string_value: Option<String> =
                        pstat_row.get("default_string_value");
                    let default_bool_value: Option<bool> = pstat_row.get("default_bool_value");
                    let visibility_str: String = pstat_row.get("visibility");

                    let pstat_id_parsed = Identifier::parse_str(&pstat_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse player stat id: {}", e))
                    })?;

                    let default_value = if let Some(v) = default_int_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Int(v)
                    } else if let Some(v) = default_float_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Float(v)
                    } else if let Some(v) = default_string_value {
                        crate::domain::game::value_objects::stat_value::StatValue::String(v)
                    } else if let Some(v) = default_bool_value {
                        crate::domain::game::value_objects::stat_value::StatValue::Bool(v)
                    } else {
                        return Err(DatabaseError::Custom(
                            "Player stat has no default value".to_string(),
                        ));
                    };

                    let visibility = match visibility_str.as_str() {
                        "Public" => crate::domain::game::value_objects::stat_visibility::PlayerStatVisibility::Public,
                        "Protected" => crate::domain::game::value_objects::stat_visibility::PlayerStatVisibility::Protected,
                        "Private" => crate::domain::game::value_objects::stat_visibility::PlayerStatVisibility::Private,
                        _ => return Err(DatabaseError::Custom(format!("Invalid player stat visibility: {}", visibility_str))),
                    };

                    let pos_str: String = pstat_row.get("pos");
                    let pos = crate::domain::common::position::Position::from_str(&pos_str)
                        .ok_or_else(|| DatabaseError::Custom(format!("Failed to parse position: {}", pos_str)))?;

                    // Load player stat values
                    let values_rows = sqlx::query(
                        r#"
                        SELECT player_id, int_value, float_value, string_value, bool_value
                        FROM player_stat_values WHERE player_stat_id = ?
                        "#,
                    )
                    .bind(&pstat_id)
                    .fetch_all(&mut *conn)
                    .await
                    .map_err(|e| {
                        DatabaseError::Custom(format!("Failed to fetch player stat values: {}", e))
                    })?;

                    let mut values = std::collections::HashMap::new();
                    for val_row in values_rows {
                        let player_id_str: String = val_row.get("player_id");
                        let int_val: Option<i64> = val_row.get("int_value");
                        let float_val: Option<f64> = val_row.get("float_value");
                        let string_val: Option<String> = val_row.get("string_value");
                        let bool_val: Option<bool> = val_row.get("bool_value");

                        let player_id = Identifier::parse_str(&player_id_str).map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse player id: {}", e))
                        })?;

                        let stat_value = if let Some(v) = int_val {
                            crate::domain::game::value_objects::stat_value::StatValue::Int(v)
                        } else if let Some(v) = float_val {
                            crate::domain::game::value_objects::stat_value::StatValue::Float(v)
                        } else if let Some(v) = string_val {
                            crate::domain::game::value_objects::stat_value::StatValue::String(v)
                        } else if let Some(v) = bool_val {
                            crate::domain::game::value_objects::stat_value::StatValue::Bool(v)
                        } else {
                            return Err(DatabaseError::Custom(
                                "Player stat value has no value".to_string(),
                            ));
                        };

                        values.insert(player_id, stat_value);
                    }

                    player_stats.push(crate::domain::game::entities::stat::PlayerStat::new_raw(
                        pstat_id_parsed,
                        pstat_name,
                        values,
                        default_value,
                        visibility,
                        pos,
                    ));
                }

                // Load actions
                let actions_rows = sqlx::query(
                    r#"
                    SELECT id, name, source_code, pos
                    FROM actions WHERE game_instance_id = ?
                    "#,
                )
                .bind(&instance_id)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch actions: {}", e)))?;

                let mut actions = Vec::new();
                for action_row in actions_rows {
                    let action_id: String = action_row.get("id");
                    let action_name: String = action_row.get("name");
                    let source_code: String = action_row.get("source_code");
                    let pos_str: String = action_row.get("pos");

                    let action_id_parsed = Identifier::parse_str(&action_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse action id: {}", e))
                    })?;

                    let pos = crate::domain::common::position::Position::from_str(&pos_str)
                        .ok_or_else(|| DatabaseError::Custom(format!("Failed to parse position: {}", pos_str)))?;

                    actions.push(crate::domain::game::entities::action::Action::new_raw(
                        action_id_parsed,
                        action_name,
                        source_code,
                        pos,
                    ));
                }

                // Load pages
                let pages_rows = sqlx::query(
                    r#"
                    SELECT id, name, source_code, pos
                    FROM pages WHERE game_instance_id = ?
                    "#,
                )
                .bind(&instance_id)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch pages: {}", e)))?;

                let mut pages = Vec::new();
                for page_row in pages_rows {
                    let page_id: String = page_row.get("id");
                    let page_name: String = page_row.get("name");
                    let source_code: String = page_row.get("source_code");
                    let pos_str: String = page_row.get("pos");

                    let page_id_parsed = Identifier::parse_str(&page_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse page id: {}", e))
                    })?;

                    let pos = crate::domain::common::position::Position::from_str(&pos_str)
                        .ok_or_else(|| DatabaseError::Custom(format!("Failed to parse position: {}", pos_str)))?;

                    pages.push(crate::domain::game::entities::page::Page::new_raw(
                        page_id_parsed,
                        page_name,
                        source_code,
                        pos,
                    ));
                }

                // Load players
                let players_rows = sqlx::query(
                    r#"
                    SELECT id, user_id
                    FROM players WHERE game_instance_id = ?
                    "#,
                )
                .bind(&instance_id)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch players: {}", e)))?;

                let mut players = Vec::new();
                for player_row in players_rows {
                    let player_id: String = player_row.get("id");
                    let user_id_opt: Option<String> = player_row.get("user_id");

                    let player_id_parsed = Identifier::parse_str(&player_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse player id: {}", e))
                    })?;

                    let user_id = if let Some(uid_str) = user_id_opt {
                        Some(Identifier::parse_str(&uid_str).map_err(|e| {
                            DatabaseError::Custom(format!("Failed to parse user id: {}", e))
                        })?)
                    } else {
                        None
                    };

                    players.push(crate::domain::game::entities::player::Player::new_raw(
                        player_id_parsed,
                        user_id,
                    ));
                }

                // Load log
                let log_row = sqlx::query(
                    r#"
                    SELECT id FROM logs WHERE game_instance_id = ?
                    "#,
                )
                .bind(&instance_id)
                .fetch_optional(&mut *conn)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to fetch log: {}", e)))?;

                let log = if let Some(log_row) = log_row {
                    let log_id: String = log_row.get("id");
                    let log_id_parsed = Identifier::parse_str(&log_id).map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse log id: {}", e))
                    })?;

                    // Load log entries
                    let entries_rows = sqlx::query(
                        r#"
                        SELECT entry, timestamp FROM log_entries WHERE log_id = ? ORDER BY timestamp ASC
                        "#,
                    )
                    .bind(&log_id)
                    .fetch_all(&mut *conn)
                    .await
                    .map_err(|e| DatabaseError::Custom(format!("Failed to fetch log entries: {}", e)))?;

                    let mut entries = Vec::new();
                    for entry_row in entries_rows {
                        let entry_json: String = entry_row.get("entry");
                        let timestamp_str: String = entry_row.get("timestamp");

                        let log_entry: crate::domain::game::entities::log::LogEntry =
                            serde_json::from_str(&entry_json).map_err(|e| {
                                DatabaseError::Custom(format!(
                                    "Failed to deserialize log entry: {}",
                                    e
                                ))
                            })?;
                        let timestamp =
                            crate::domain::common::date_time::DateTime::parse_str(&timestamp_str)
                                .map_err(|e| {
                                DatabaseError::Custom(format!(
                                    "Failed to parse log timestamp: {}",
                                    e
                                ))
                            })?;

                        entries.push((log_entry, timestamp));
                    }

                    crate::domain::game::entities::log::Log::new_raw(log_id_parsed, entries)
                } else {
                    crate::domain::game::entities::log::Log::new()
                };

                // Construct GameInstance
                let game_instance = GameInstance::new_raw(
                    instance_id_parsed,
                    name,
                    current_player_index as usize,
                    round as u32,
                    game_stats,
                    player_stats,
                    actions,
                    pages,
                    players,
                    log,
                    source_game,
                    gm_user_id,
                    created_at,
                    last_played_at,
                );

                Ok(Some(game_instance))
            }
            None => Ok(None),
        }
    }

    async fn list_by_game_id(
        &self,
        game_id: Identifier,
    ) -> Result<Vec<GameInstanceMetadataProjection>, DatabaseError> {
        let mut conn =
            self.db.acquire().await.map_err(|e| {
                DatabaseError::Custom(format!("Failed to acquire connection: {}", e))
            })?;

        let game_id_str = game_id.to_string();

        let rows = sqlx::query(
            r#"
            SELECT 
                gi.id,
                gi.name,
                gi.source_game as game_id,
                gi.round as current_round,
                gi.gm_user_id,
                gi.created_at,
                gi.last_played_at,
                COUNT(p.id) as player_count
            FROM game_instances gi
            LEFT JOIN players p ON gi.id = p.game_instance_id
            WHERE gi.source_game = ?
            GROUP BY gi.id
            ORDER BY gi.last_played_at DESC
            "#,
        )
        .bind(&game_id_str)
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| DatabaseError::Custom(format!("Failed to fetch game instances: {}", e)))?;

        let mut projections = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            let name: String = row.get("name");
            let game_id_str: String = row.get("game_id");
            let current_round: i64 = row.get("current_round");
            let gm_user_id_str: String = row.get("gm_user_id");
            let created_at_str: String = row.get("created_at");
            let last_played_at_str: String = row.get("last_played_at");
            let player_count: i64 = row.get("player_count");

            let id = Identifier::parse_str(&id_str)
                .map_err(|e| DatabaseError::Custom(format!("Failed to parse id: {}", e)))?;
            let game_id = Identifier::parse_str(&game_id_str)
                .map_err(|e| DatabaseError::Custom(format!("Failed to parse game_id: {}", e)))?;
            let gm_user_id = Identifier::parse_str(&gm_user_id_str)
                .map_err(|e| DatabaseError::Custom(format!("Failed to parse gm_user_id: {}", e)))?;
            let created_at = crate::domain::common::date_time::DateTime::parse_str(&created_at_str)
                .map_err(|e| DatabaseError::Custom(format!("Failed to parse created_at: {}", e)))?;
            let last_played_at =
                crate::domain::common::date_time::DateTime::parse_str(&last_played_at_str)
                    .map_err(|e| {
                        DatabaseError::Custom(format!("Failed to parse last_played_at: {}", e))
                    })?;

            projections.push(GameInstanceMetadataProjection {
                id,
                name,
                game_id,
                player_count: player_count as usize,
                current_round: current_round as u32,
                gm_user_id,
                created_at,
                last_played_at,
            });
        }

        Ok(projections)
    }

    async fn save(&self, game_instance: &GameInstance) -> Result<(), DatabaseError> {
        let mut conn =
            self.db.acquire().await.map_err(|e| {
                DatabaseError::Custom(format!("Failed to acquire connection: {}", e))
            })?;

        let mut tx = conn
            .begin()
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to begin transaction: {}", e)))?;

        let id = game_instance.id().to_string();
        let name = game_instance.name();
        let current_player_index = game_instance.current_player_index() as i64;
        let round = game_instance.round() as i64;
        let gm_user_id = game_instance.gm_user_id().to_string();
        let source_game = game_instance.source_game().id().to_string();
        let created_at = game_instance.created_at().to_string();
        let last_played_at = game_instance.last_played_at().to_string();

        // Insert or replace the game instance
        sqlx::query(
            "INSERT OR REPLACE INTO game_instances (id, name, current_player_index, round, gm_user_id, source_game, created_at, last_played_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&name)
        .bind(current_player_index)
        .bind(round)
        .bind(&gm_user_id)
        .bind(&source_game)
        .bind(&created_at)
        .bind(&last_played_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| DatabaseError::Custom(format!("Failed to save game instance: {}", e)))?;

        // Save players FIRST - before player stats (foreign key constraint)
        sqlx::query("DELETE FROM players WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete players: {}", e)))?;

        for player in game_instance.players() {
            let player_id = player.id().to_string();
            let user_id = player.user_id().map(|uid| uid.to_string());

            sqlx::query("INSERT INTO players (id, game_instance_id, user_id) VALUES (?, ?, ?)")
                .bind(&player_id)
                .bind(&id)
                .bind(&user_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to save player: {}", e)))?;
        }

        // Save game stats
        sqlx::query("DELETE FROM game_stats WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete game stats: {}", e)))?;

        for stat in game_instance.game_stats() {
            let stat_id = stat.id().to_string();
            let stat_name = stat.name();
            let visibility = stat.visibility().to_string();

            // Extract value columns
            let (val_int, val_float, val_str, val_bool) = match stat.value() {
                crate::domain::game::value_objects::stat_value::StatValue::Int(v) => {
                    (Some(*v), None, None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Float(v) => {
                    (None, Some(*v), None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::String(v) => {
                    (None, None, Some(v.clone()), None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Bool(v) => {
                    (None, None, None, Some(*v))
                }
            };

            // Extract default value columns
            let (def_int, def_float, def_str, def_bool) = match stat.default() {
                crate::domain::game::value_objects::stat_value::StatValue::Int(v) => {
                    (Some(*v), None, None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Float(v) => {
                    (None, Some(*v), None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::String(v) => {
                    (None, None, Some(v.clone()), None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Bool(v) => {
                    (None, None, None, Some(*v))
                }
            };

            let pos_str = stat.pos().to_string();

            sqlx::query(
                "INSERT INTO game_stats (id, game_instance_id, name, int_value, float_value, string_value, bool_value, default_int_value, default_float_value, default_string_value, default_bool_value, visibility, pos) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&stat_id)
            .bind(&id)
            .bind(&stat_name)
            .bind(val_int)
            .bind(val_float)
            .bind(val_str)
            .bind(val_bool)
            .bind(def_int)
            .bind(def_float)
            .bind(def_str)
            .bind(def_bool)
            .bind(&visibility)
            .bind(&pos_str)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save game stat: {}", e)))?;
        }

        // Save player stats
        sqlx::query("DELETE FROM player_stats WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete player stats: {}", e)))?;

        sqlx::query("DELETE FROM player_stat_values WHERE player_stat_id IN (SELECT id FROM player_stats WHERE game_instance_id = ?)")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete player stat values: {}", e)))?;

        for player_stat in game_instance.player_stats() {
            let pstat_id = player_stat.id().to_string();
            let pstat_name = player_stat.name();
            let visibility = player_stat.visibility().to_string();

            // Extract default value columns
            let (def_int, def_float, def_str, def_bool) = match player_stat.default() {
                crate::domain::game::value_objects::stat_value::StatValue::Int(v) => {
                    (Some(*v), None, None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Float(v) => {
                    (None, Some(*v), None, None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::String(v) => {
                    (None, None, Some(v.clone()), None)
                }
                crate::domain::game::value_objects::stat_value::StatValue::Bool(v) => {
                    (None, None, None, Some(*v))
                }
            };

            let pos_str = player_stat.pos().to_string();

            sqlx::query(
                "INSERT INTO player_stats (id, game_instance_id, name, default_int_value, default_float_value, default_string_value, default_bool_value, visibility, pos) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&pstat_id)
            .bind(&id)
            .bind(&pstat_name)
            .bind(def_int)
            .bind(def_float)
            .bind(def_str)
            .bind(def_bool)
            .bind(&visibility)
            .bind(&pos_str)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save player stat: {}", e)))?;

            // Save player stat values
            for (player_id, stat_value) in player_stat.values() {
                let (val_int, val_float, val_str, val_bool) = match stat_value {
                    crate::domain::game::value_objects::stat_value::StatValue::Int(v) => {
                        (Some(*v), None, None, None)
                    }
                    crate::domain::game::value_objects::stat_value::StatValue::Float(v) => {
                        (None, Some(*v), None, None)
                    }
                    crate::domain::game::value_objects::stat_value::StatValue::String(v) => {
                        (None, None, Some(v.clone()), None)
                    }
                    crate::domain::game::value_objects::stat_value::StatValue::Bool(v) => {
                        (None, None, None, Some(*v))
                    }
                };

                sqlx::query(
                    "INSERT INTO player_stat_values (player_stat_id, player_id, int_value, float_value, string_value, bool_value) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&pstat_id)
                .bind(player_id.to_string())
                .bind(val_int)
                .bind(val_float)
                .bind(val_str)
                .bind(val_bool)
                .execute(&mut *tx)
                .await
                .map_err(|e| DatabaseError::Custom(format!("Failed to save player stat value: {}", e)))?;
            }
        }

        // Save actions
        sqlx::query("DELETE FROM actions WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete actions: {}", e)))?;

        for action in game_instance.actions() {
            let action_id = action.id().to_string();
            let action_name = action.name();
            let source_code = action.source_code();
            let pos = action.pos().to_string();

            sqlx::query(
                "INSERT INTO actions (id, game_instance_id, name, source_code, pos) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&action_id)
            .bind(&id)
            .bind(&action_name)
            .bind(&source_code)
            .bind(&pos)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save action: {}", e)))?;
        }

        // Save pages
        sqlx::query("DELETE FROM pages WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete pages: {}", e)))?;

        for page in game_instance.pages() {
            let page_id = page.id().to_string();
            let page_name = page.name();
            let source_code = page.source_code();
            let pos = page.pos().to_string();

            sqlx::query(
                "INSERT INTO pages (id, game_instance_id, name, source_code, pos) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&page_id)
            .bind(&id)
            .bind(&page_name)
            .bind(&source_code)
            .bind(&pos)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save page: {}", e)))?;
        }

        // Save log
        sqlx::query("DELETE FROM log_entries WHERE log_id IN (SELECT id FROM logs WHERE game_instance_id = ?)")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete log entries: {}", e)))?;

        sqlx::query("DELETE FROM logs WHERE game_instance_id = ?")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete log: {}", e)))?;

        let log_id = game_instance.log().id().to_string();
        sqlx::query("INSERT INTO logs (id, game_instance_id) VALUES (?, ?)")
            .bind(&log_id)
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save log: {}", e)))?;

        for (entry, timestamp) in game_instance.log().entries() {
            let entry_id = Identifier::new().to_string();
            let entry_json = serde_json::to_string(entry).map_err(|e| {
                DatabaseError::Custom(format!("Failed to serialize log entry: {}", e))
            })?;
            let timestamp_str = timestamp.to_string();

            sqlx::query(
                "INSERT INTO log_entries (id, log_id, entry, timestamp) VALUES (?, ?, ?, ?)",
            )
            .bind(&entry_id)
            .bind(&log_id)
            .bind(&entry_json)
            .bind(&timestamp_str)
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to save log entry: {}", e)))?;
        }

        tx.commit()
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to commit transaction: {}", e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        _game_id: Identifier,
        instance_id: Identifier,
    ) -> Result<(), DatabaseError> {
        let mut conn =
            self.db.acquire().await.map_err(|e| {
                DatabaseError::Custom(format!("Failed to acquire connection: {}", e))
            })?;

        let instance_id_str = instance_id.to_string();
        sqlx::query("DELETE FROM game_instances WHERE id = ?")
            .bind(&instance_id_str)
            .execute(&mut *conn)
            .await
            .map_err(|e| DatabaseError::Custom(format!("Failed to delete game instance: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        application::{
            game::contracts::GameRepositoryContract, user::contracts::UserRepositoryContract,
        },
        domain::{
            common::date_time::DateTime,
            game::{
                entities::{
                    action::Action,
                    game::Game,
                    game_instance::GameInstance,
                    log::Log,
                    page::Page,
                    player::Player,
                    stat::{GameStat, PlayerStat},
                },
                value_objects::{
                    stat_value::StatValue,
                    stat_visibility::{GameStatVisibility, PlayerStatVisibility},
                },
            },
            user::entities::User,
        },
        infrastructure::persistence::{
            db::create_test_pool,
            repositories::{game::SqliteGameRepository, user::SqliteUserRepository},
        },
    };

    use super::*;

    async fn create_and_save_user(user_repo: &SqliteUserRepository, name: String) -> User {
        let user = User::try_new(Identifier::new(), name, "testpassword".to_string()).unwrap();
        user_repo.save(&user).await.unwrap();
        user
    }

    async fn create_and_save_game(game_repo: &SqliteGameRepository) -> Game {
        let game = Game::new_raw(
            Identifier::new(),
            "Test Game".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );
        game_repo.save(&game).await.unwrap();
        game
    }

    async fn create_game_instance(i: u16, game: Game, user1: User, user2: User) -> GameInstance {
        let player_id_1 = Identifier::new();
        let player_id_2 = Identifier::new();

        let test_action = Action::new_raw(
            Identifier::new(),
            "Test Action".to_string(),
            "print('This is a test action')".to_string(),
            crate::domain::common::position::Position::new(0, 0),
        );

        let mut log = Log::new();
        log.log_action(
            user1.id().clone(),
            test_action.id().clone(),
            "Test payload".to_string(),
        );
        log.log_error("Something went wrong.".to_string());
        log.log_system("Hi from the system.".to_string());

        GameInstance::new_raw(
            Identifier::new(),
            format!("Test Game Instance {}", i),
            1,
            5,
            vec![
                GameStat::new_raw(
                    Identifier::new(),
                    "Score".to_string(),
                    StatValue::Int(0),
                    StatValue::Int(100),
                    GameStatVisibility::Public,
                    crate::domain::common::position::Position::new(0, 0),
                ),
                GameStat::new_raw(
                    Identifier::new(),
                    "Gold".to_string(),
                    StatValue::Int(30),
                    StatValue::Int(5),
                    GameStatVisibility::Private,
                    crate::domain::common::position::Position::new(0, 0),
                ),
            ],
            vec![
                PlayerStat::new_raw(
                    Identifier::new(),
                    "Health".to_string(),
                    [
                        (player_id_1, StatValue::Int(100)),
                        (player_id_2, StatValue::Int(50)),
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>(),
                    StatValue::Int(0),
                    PlayerStatVisibility::Public,
                    crate::domain::common::position::Position::new(0, 0),
                ),
                PlayerStat::new_raw(
                    Identifier::new(),
                    "Money".to_string(),
                    [
                        (player_id_1, StatValue::Int(20)),
                        (player_id_2, StatValue::Int(10)),
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>(),
                    StatValue::Int(0),
                    PlayerStatVisibility::Protected,
                    crate::domain::common::position::Position::new(0, 0),
                ),
            ],
            vec![test_action],
            vec![Page::new_raw(
                Identifier::new(),
                "testPage".to_string(),
                "page testPage {}".to_string(),
                crate::domain::common::position::Position::new(0, 0),
            )],
            vec![
                Player::new_raw(player_id_1, None),
                Player::new_raw(player_id_2, Some(user2.id().clone())),
            ],
            log,
            game,
            user1.id().clone(),
            DateTime::now(),
            DateTime::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_get_by_id() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create a game instance
        let game_instance = create_game_instance(1, game, user1, user2).await;

        // Save the game instance
        repository.save(&game_instance).await.unwrap();

        // Retrieve the game instance by ID
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_some());
        let retrieved_instance = retrieved_instance.unwrap();

        // Check key fields
        assert_eq!(retrieved_instance.id(), game_instance.id());
        assert_eq!(retrieved_instance.name(), game_instance.name());
        assert_eq!(
            retrieved_instance.current_player_index(),
            game_instance.current_player_index()
        );
        assert_eq!(retrieved_instance.round(), game_instance.round());
        assert_eq!(retrieved_instance.gm_user_id(), game_instance.gm_user_id());
        assert_eq!(
            retrieved_instance.source_game().id(),
            game_instance.source_game().id()
        );
        assert_eq!(retrieved_instance.created_at(), game_instance.created_at());
        assert_eq!(
            retrieved_instance.last_played_at(),
            game_instance.last_played_at()
        );

        // Check game stats
        assert_eq!(
            retrieved_instance.game_stats().len(),
            game_instance.game_stats().len()
        );
        for (retrieved_stat, original_stat) in retrieved_instance
            .game_stats()
            .iter()
            .zip(game_instance.game_stats().iter())
        {
            assert_eq!(retrieved_stat.id(), original_stat.id());
            assert_eq!(retrieved_stat.name(), original_stat.name());
            assert_eq!(retrieved_stat.value(), original_stat.value());
            assert_eq!(retrieved_stat.default(), original_stat.default());
            assert_eq!(retrieved_stat.visibility(), original_stat.visibility());
        }

        // Check player stats (HashMap ordering might differ, so check by name and values)
        assert_eq!(
            retrieved_instance.player_stats().len(),
            game_instance.player_stats().len()
        );
        for original_pstat in game_instance.player_stats().iter() {
            let retrieved_pstat = retrieved_instance
                .player_stats()
                .iter()
                .find(|p| p.name() == original_pstat.name())
                .expect(&format!("Player stat {} not found", original_pstat.name()));

            assert_eq!(retrieved_pstat.id(), original_pstat.id());
            assert_eq!(retrieved_pstat.default(), original_pstat.default());
            assert_eq!(retrieved_pstat.visibility(), original_pstat.visibility());

            // Check all player stat values match
            assert_eq!(
                retrieved_pstat.values().len(),
                original_pstat.values().len()
            );
            for (player_id, value) in original_pstat.values().iter() {
                assert_eq!(
                    retrieved_pstat.values().get(player_id),
                    Some(value),
                    "Player stat value mismatch for player {}",
                    player_id
                );
            }
        }

        // Check actions
        assert_eq!(
            retrieved_instance.actions().len(),
            game_instance.actions().len()
        );
        for (retrieved_action, original_action) in retrieved_instance
            .actions()
            .iter()
            .zip(game_instance.actions().iter())
        {
            assert_eq!(retrieved_action.id(), original_action.id());
            assert_eq!(retrieved_action.name(), original_action.name());
            assert_eq!(
                retrieved_action.source_code(),
                original_action.source_code()
            );
            assert_eq!(retrieved_action.pos(), original_action.pos());
        }

        // Check pages
        assert_eq!(
            retrieved_instance.pages().len(),
            game_instance.pages().len()
        );
        for (retrieved_page, original_page) in retrieved_instance
            .pages()
            .iter()
            .zip(game_instance.pages().iter())
        {
            assert_eq!(retrieved_page.id(), original_page.id());
            assert_eq!(retrieved_page.name(), original_page.name());
            assert_eq!(retrieved_page.source_code(), original_page.source_code());
            assert_eq!(retrieved_page.pos(), original_page.pos());
        }

        // Check players
        assert_eq!(
            retrieved_instance.players().len(),
            game_instance.players().len()
        );
        for (retrieved_player, original_player) in retrieved_instance
            .players()
            .iter()
            .zip(game_instance.players().iter())
        {
            assert_eq!(retrieved_player.id(), original_player.id());
            assert_eq!(retrieved_player.user_id(), original_player.user_id());
        }

        // Check log
        assert_eq!(retrieved_instance.log(), game_instance.log());
    }

    #[tokio::test]
    async fn test_list_by_game_id() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create and save multiple game instances
        for i in 0..5 {
            let game_instance =
                create_game_instance(i, game.clone(), user1.clone(), user2.clone()).await;
            repository.save(&game_instance).await.unwrap();
        }

        // List game instances by game ID
        let instances_metadata = repository.list_by_game_id(game.id().clone()).await.unwrap();

        assert_eq!(instances_metadata.len(), 5);
        for i in 0..5 {
            assert!(
                instances_metadata
                    .iter()
                    .any(|meta| meta.name == format!("Test Game Instance {}", i))
            ); // Each instance should have the correct name
        }
        assert!(
            instances_metadata
                .iter()
                .all(|meta| &meta.game_id == game.id())
        ); // All instances should belong to the same game
        assert!(instances_metadata.iter().all(|g_i| g_i.current_round == 5));
        assert!(instances_metadata.iter().all(|g_i| g_i.player_count == 2));

        // Check order
        for i in 0..4 {
            assert!(
                instances_metadata[i].last_played_at >= instances_metadata[i + 1].last_played_at
            ); // Instances should be ordered by last_played_at descending
        }
    }

    #[tokio::test]
    async fn test_delete() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create a game instance
        let game_instance = create_game_instance(1, game, user1, user2).await;

        // Save the game instance
        repository.save(&game_instance).await.unwrap();

        // Check if the instance exists before deletion
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();
        assert!(retrieved_instance.is_some()); // The instance should exist before deletion

        // Delete the game instance
        repository
            .delete(
                game_instance.source_game().id().clone(),
                game_instance.id().clone(),
            )
            .await
            .unwrap();

        // Try to retrieve the deleted game instance by ID
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_none()); // The instance should no longer exist
    }
}
