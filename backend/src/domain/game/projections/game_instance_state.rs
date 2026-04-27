use crate::domain::{common::identifier::Identifier, game::value_objects::data::VariableValue};

pub struct StatValueProjection {
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
    pub str_value: Option<String>,
    pub bool_value: Option<bool>,
}

impl From<VariableValue> for StatValueProjection {
    fn from(value: VariableValue) -> Self {
        match value {
            VariableValue::Int(v) => StatValueProjection {
                int_value: Some(v),
                float_value: None,
                str_value: None,
                bool_value: None,
            },
            VariableValue::Float(v) => StatValueProjection {
                int_value: None,
                float_value: Some(v),
                str_value: None,
                bool_value: None,
            },
            VariableValue::String(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: Some(v),
                bool_value: None,
            },
            VariableValue::Bool(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: None,
                bool_value: Some(v),
            },
        }
    }
}

pub struct GameStatProjection {
    pub name: String,
    pub value: StatValueProjection,
    pub default: StatValueProjection,
    pub visibility: String,
}

pub struct PlayerStatProjection {
    pub name: String,
    pub values: Vec<(String, StatValueProjection)>, // Vec of (player_name, value)
    pub default: StatValueProjection,
    pub visibility: String,
}

pub struct PlayerProjection {
    pub name: String,
    pub user_id: Option<Identifier>,
}

pub struct GameInstanceStateProjection {
    pub round: u32,
    pub current_player_index: usize,
    pub game_stats: Vec<GameStatProjection>,
    pub player_stats: Vec<PlayerStatProjection>,
    pub players: Vec<PlayerProjection>,
}
