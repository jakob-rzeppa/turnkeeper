use crate::domain::entity::game::Game;

/// The GameHandler is the central manager of the game.
///
/// It will be created when starting / resuming a game and is persisted while the game is running.
pub struct GameHandler {
    instance: Game,
}