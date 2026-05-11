use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::{
        common::position::Positioned,
        game::{
            abstract_syntax_tree::statement::pset_statement::PSetStatement,
            value_objects::data::Value,
        },
    },
};

impl Executable<()> for PSetStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let player = match self.player().execute(env).await? {
            Value::String(p) => p,
            val => {
                return Err(RuntimeError::TypeMismatch {
                    expected: "string for the player name".to_string(),
                    found: val,
                    pos: self.position(),
                });
            }
        };

        let value = self.value().execute(env).await?;

        env.set_player_stat(self.stat(), &player, value)?;

        Ok(())
    }
}
