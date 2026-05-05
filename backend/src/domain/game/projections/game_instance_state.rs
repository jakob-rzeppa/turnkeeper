use serde::{Deserialize, Serialize};

use crate::domain::{common::identifier::Identifier, game::value_objects::data::Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatValueProjection {
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
    pub str_value: Option<String>,
    pub bool_value: Option<bool>,
}

impl From<Value> for StatValueProjection {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(v) => StatValueProjection {
                int_value: Some(v),
                float_value: None,
                str_value: None,
                bool_value: None,
            },
            Value::Float(v) => StatValueProjection {
                int_value: None,
                float_value: Some(v),
                str_value: None,
                bool_value: None,
            },
            Value::String(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: Some(v),
                bool_value: None,
            },
            Value::Bool(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: None,
                bool_value: Some(v),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatStateProjection {
    pub name: String,
    pub value: StatValueProjection,
    pub default: StatValueProjection,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatStateProjection {
    pub name: String,
    pub values: Vec<(String, StatValueProjection)>, // Vec of (player_name, value)
    pub default: StatValueProjection,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProjection {
    pub name: String,
    pub user_id: Option<Identifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInstanceStateProjection {
    pub round: u32,
    pub current_player_index: usize,
    pub game_stats: Vec<GameStatStateProjection>,
    pub player_stats: Vec<PlayerStatStateProjection>,
    pub players: Vec<PlayerProjection>,
}
