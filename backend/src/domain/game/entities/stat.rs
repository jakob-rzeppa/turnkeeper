use std::collections::HashMap;

use crate::domain::{
    common::{identifier::Identifier, position::Position},
    game::{
        error::GameInstanceError,
        value_objects::{
            stat_value::StatValue,
            stat_visibility::{GameStatVisibility, PlayerStatVisibility},
        },
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct GameStat {
    id: Identifier,

    name: String,
    value: StatValue,

    default: StatValue,
    visibility: GameStatVisibility,

    pos: Position,
}

impl GameStat {
    /// Creates a new game stat with the given name, default value, and visibility.
    ///
    /// The `value` is initialized to the `default` value, and can be changed later using `set_value`.
    pub fn new(
        name: String,
        default: StatValue,
        visibility: GameStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            id: Identifier::new(),
            name,
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
        id: Identifier,
        name: String,
        value: StatValue,
        default: StatValue,
        visibility: GameStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            id,
            name,
            value,
            default,
            visibility,
            pos,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &StatValue {
        &self.value
    }

    pub fn default(&self) -> &StatValue {
        &self.default
    }

    pub fn visibility(&self) -> &GameStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn set_value(&mut self, value: StatValue) {
        self.value = value;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerStat {
    id: Identifier,

    name: String,
    values: HashMap<Identifier, StatValue>, // player_id -> value

    default: StatValue,
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
        default: StatValue,
        visibility: PlayerStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            id: Identifier::new(),
            name,
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
        id: Identifier,
        name: String,
        values: HashMap<Identifier, StatValue>,
        default: StatValue,
        visibility: PlayerStatVisibility,
        pos: Position,
    ) -> Self {
        Self {
            id,
            name,
            values,
            default,
            visibility,
            pos,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &HashMap<Identifier, StatValue> {
        &self.values
    }

    pub fn default(&self) -> &StatValue {
        &self.default
    }

    pub fn visibility(&self) -> &PlayerStatVisibility {
        &self.visibility
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn get_owning_player_value(&self, player_id: &Identifier) -> Option<&StatValue> {
        self.values.get(player_id)
    }

    pub fn set_value_for_player(
        &mut self,
        player_id: &Identifier,
        value: StatValue,
    ) -> Result<(), GameInstanceError> {
        if let None = self.values.get(player_id) {
            return Err(GameInstanceError::PlayerInStatNotFound {
                stat_id: self.id.clone(),
                player_id: player_id.clone(),
                stat_name: self.name.clone(),
            });
        }

        self.values.insert(player_id.clone(), value);
        Ok(())
    }
}
