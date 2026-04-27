use std::collections::HashMap;

use crate::domain::{
    common::position::Position,
    game::{
        error::GameInstanceError,
        value_objects::{
            data::{VariableType, VariableValue},
            visibility::{GameStatVisibility, PlayerStatVisibility},
        },
    },
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameStat {
    name: String,
    datatype: VariableType,
    value: VariableValue,

    default: VariableValue,
    visibility: GameStatVisibility,

    pos: Position,
}

impl GameStat {
    /// Creates a new game stat with the given name, default value, and visibility.
    ///
    /// The `value` is initialized to the `default` value, and can be changed later using `set_value`.
    pub fn new(
        name: String,
        datatype: VariableType,
        default: VariableValue,
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
        datatype: VariableType,
        value: VariableValue,
        default: VariableValue,
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

    pub fn datatype(&self) -> &VariableType {
        &self.datatype
    }

    pub fn value(&self) -> &VariableValue {
        &self.value
    }

    pub fn default(&self) -> &VariableValue {
        &self.default
    }

    pub fn visibility(&self) -> &GameStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn set_value(&mut self, value: VariableValue) {
        self.value = value;
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerStat {
    name: String,
    datatype: VariableType,
    values: HashMap<String, VariableValue>, // player_name -> value

    default: VariableValue,
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
        datatype: VariableType,
        default: VariableValue,
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
        datatype: VariableType,
        values: HashMap<String, VariableValue>, // player_name -> value
        default: VariableValue,
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

    pub fn datatype(&self) -> &VariableType {
        &self.datatype
    }

    pub fn values(&self) -> &HashMap<String, VariableValue> {
        &self.values
    }

    pub fn default(&self) -> &VariableValue {
        &self.default
    }

    pub fn visibility(&self) -> &PlayerStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn get_owning_player_value(&self, player_name: &String) -> Option<&VariableValue> {
        self.values.get(player_name)
    }

    pub fn set_value_for_player(
        &mut self,
        player_name: &str,
        value: VariableValue,
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
}
