use crate::application::game::plugin::parser::parse_source_code;

mod common;
mod lexer;
mod parser;
mod runtime;

#[allow(dead_code)]
pub struct Plugin {
    pub source_code: String,
}

impl Plugin {
    #[allow(dead_code)]
    pub fn new(source_code: String) -> Self {
        Self { source_code }
    }

    #[allow(dead_code)]
    pub fn execute(&self) -> Result<(), anyhow::Error> {
        let mut runtime_env = runtime::RuntimeEnvironment::new();

        let abstract_syntax_tree = parse_source_code(&self.source_code)?;

        // Runtime execution
        runtime_env.run(&abstract_syntax_tree)?;

        Ok(())
    }
}
