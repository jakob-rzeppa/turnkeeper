use crate::application::game::plugin::{
    lexer::tokenize,
    parser::abstract_syntax_tree::{Parsable, TokenStream, error::ParsingError, root::Root},
};

pub mod abstract_syntax_tree;

pub fn parse_source_code(source: &str) -> Result<Root, ParsingError> {
    let tokens = tokenize(source);
    let mut token_stream = TokenStream::new(tokens);

    let root = Root::parse(&mut token_stream)?;

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = "let x: int = 42;\nx = x + 4;";

        let result = parse_source_code(code);
        match &result {
            Ok(root) => println!("{:#?}", root),
            Err(err) => println!("{}", err.context_message(code)),
        }
    }
}
