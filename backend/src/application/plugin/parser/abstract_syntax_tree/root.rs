use crate::application::plugin::parser::abstract_syntax_tree::{
    Parsable, ParsingError, TokenStream, statement::Statement,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    statements: Vec<Statement>,
}

impl Root {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Parsable for Root {
    fn is_next(_: &TokenStream) -> bool {
        true // Root is the entry point, so we always return true here
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let mut statements = Vec::new();
        while ts.peek().is_some() {
            statements.push(Statement::parse(ts)?);
        }
        Ok(Root { statements })
    }
}
