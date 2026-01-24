use crate::domain::entity::game::Game;

/// The GameHandler is the manager of the game during the game.
///
/// It will be created when starting / resuming a game and is persisted while the game is running.
pub struct ActiveGameHandler {
    instance: Game,
}