use crate::domain::{
    common::identifier::Identifier,
    game::{entities::game_instance::GameInstance, error::GameInstanceError},
};

impl GameInstance {
    /// Adds a new player to the game with the specified ID.
    ///
    /// # Invariants
    ///
    /// - The `id` must be unique among all players in the game.
    /// - The new player should be added to all existing tradables with a default value.
    pub fn add_player(&mut self, id: Identifier) -> Result<(), GameInstanceError> {
        unimplemented!()
    }

    /// Reorders players to match the given list of UUIDs.
    ///
    /// # Errors
    ///
    /// Returns [`GameInstanceError::InvalidPlayerOrder`] if the list length differs
    /// from the current player count, contains duplicates, or references
    /// unknown player IDs.
    pub fn change_player_order(
        &mut self,
        ids_in_order: Vec<Identifier>,
    ) -> Result<(), GameInstanceError> {
        unimplemented!()
    }

    /// Attaches a user to a player by their IDs.
    ///
    /// # Invariants
    ///
    /// - A user can only be attached to one player at a time.
    /// - The player must exist in the game.
    /// - A user can only be attached if there is no other player already attached to that user.
    ///
    /// The user is not validated here. If a user doesn't exist, it will be displayed as "User not found" in the UI, but it won't cause an error at this stage, since we don't care about the user details.
    pub fn attach_user_to_player(
        &mut self,
        user_id: Identifier,
        player_id: Identifier,
    ) -> Result<(), GameInstanceError> {
        unimplemented!()
    }

    /// Detaches any user from the specified player.
    pub fn detach_user_from_player(
        &mut self,
        player_id: Identifier,
    ) -> Result<(), GameInstanceError> {
        unimplemented!()
    }
}
