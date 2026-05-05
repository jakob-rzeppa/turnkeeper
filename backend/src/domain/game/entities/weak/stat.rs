use std::collections::HashMap;

use crate::domain::{
    common::position::Position,
    game::{
        error::GameInstanceError,
        projections::stat::{GameStatMetadataProjection, PlayerStatMetadataProjection},
        value_objects::{
            data::{Datatype, Value},
            visibility::{GameStatVisibility, PlayerStatVisibility},
        },
    },
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameStat {
    name: String,
    datatype: Datatype,
    value: Value,

    default: Value,
    visibility: GameStatVisibility,

    pos: Position,
}

impl GameStat {
    /// Creates a new game stat with the given name, default value, and visibility.
    ///
    /// The `value` is initialized to the `default` value, and can be changed later using `set_value`.
    pub fn new(
        name: String,
        datatype: Datatype,
        default: Value,
        visibility: GameStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            name,
            datatype,
            value: default.clone(),
            default,
            visibility,
            pos,
        }
    }

    /// Creates a new game stat with the given raw values.
    ///
    /// This should only be used when loading a game instance from storage, where we already have the value for the stat.
    pub fn new_raw(
        name: String,
        datatype: Datatype,
        value: Value,
        default: Value,
        visibility: GameStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            name,
            datatype,
            value,
            default,
            visibility,
            pos,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn default(&self) -> &Value {
        &self.default
    }

    pub fn visibility(&self) -> &GameStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn get_metadata_projection(&self) -> GameStatMetadataProjection {
        GameStatMetadataProjection {
            name: self.name.clone(),
            datatype: self.datatype.clone(),
            default: self.default.clone(),
            visibility: self.visibility.clone(),
            pos: self.pos.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerStat {
    name: String,
    datatype: Datatype,
    values: HashMap<String, Value>, // player_name -> value

    default: Value,
    visibility: PlayerStatVisibility,

    pos: Position,
}

impl PlayerStat {
    /// Creates a new player stat with the given name, default value, and visibility.
    ///
    /// The `values` map is initialized as empty, and values for each player will be set when they are added to the game.
    ///
    /// This should only be used when creating a new game instance.
    pub fn new(
        name: String,
        datatype: Datatype,
        default: Value,
        visibility: PlayerStatVisibility,
        pos: Position,
    ) -> Self {
        assert!(
            default.is_type(&datatype),
            "Default value must match the datatype of the stat"
        );

        Self {
            name,
            datatype,
            values: HashMap::new(), // Values will be set for each player when they are added to the game
            default,
            visibility,
            pos,
        }
    }

    /// Creates a new player stat with the given raw values.
    ///
    /// This should only be used when loading a game instance from storage, where we already have the values for each player.
    pub fn new_raw(
        name: String,
        datatype: Datatype,
        values: HashMap<String, Value>, // player_name -> value
        default: Value,
        visibility: PlayerStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            name,
            datatype,
            values,
            default,
            visibility,
            pos,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    pub fn values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    pub fn default(&self) -> &Value {
        &self.default
    }

    pub fn visibility(&self) -> &PlayerStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn get_owning_player_value(&self, player_name: &String) -> Option<&Value> {
        self.values.get(player_name)
    }

    pub fn set_value_for_player(
        &mut self,
        player_name: &str,
        value: Value,
    ) -> Result<(), GameInstanceError> {
        if let None = self.values.get(player_name) {
            return Err(GameInstanceError::PlayerInStatNotFound {
                player_name: player_name.to_string(),
                stat_name: self.name.clone(),
            });
        }

        self.values.insert(player_name.to_string(), value);
        Ok(())
    }

    pub fn get_metadata_projection(&self) -> PlayerStatMetadataProjection {
        PlayerStatMetadataProjection {
            name: self.name.clone(),
            datatype: self.datatype.clone(),
            default: self.default.clone(),
            visibility: self.visibility.clone(),
            pos: self.pos.clone(),
        }
    }
}
