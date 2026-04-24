use crate::domain::{common::identifier::Identifier, game::value_objects::stat_value::StatValue};

pub struct StatValueProjection {
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
    pub str_value: Option<String>,
    pub bool_value: Option<bool>,
}

impl From<StatValue> for StatValueProjection {
    fn from(value: StatValue) -> Self {
        match value {
            StatValue::Int(v) => StatValueProjection {
                int_value: Some(v),
                float_value: None,
                str_value: None,
                bool_value: None,
            },
            StatValue::Float(v) => StatValueProjection {
                int_value: None,
                float_value: Some(v),
                str_value: None,
                bool_value: None,
            },
            StatValue::String(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: Some(v),
                bool_value: None,
            },
            StatValue::Bool(v) => StatValueProjection {
                int_value: None,
                float_value: None,
                str_value: None,
                bool_value: Some(v),
            },
        }
    }
}

pub struct GameStatProjection {
    pub id: Identifier,
    pub name: String,
    pub value: StatValueProjection,
    pub default: StatValueProjection,
    pub visibility: String,
}

pub struct PlayerStatProjection {
    pub id: Identifier,
    pub name: String,
    pub values: Vec<(Identifier, StatValueProjection)>, // Vec of (player_id, value)
    pub default: StatValueProjection,
    pub visibility: String,
}

pub struct PlayerProjection {
    pub id: Identifier,
    pub user_id: Option<Identifier>,
}

pub struct GameInstanceStateProjection {
    pub round: u32,
    pub current_player_index: usize,
    pub game_stats: Vec<GameStatProjection>,
    pub player_stats: Vec<PlayerStatProjection>,
    pub players: Vec<PlayerProjection>,
}
