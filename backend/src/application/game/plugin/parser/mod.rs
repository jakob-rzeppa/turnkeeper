use crate::application::game::plugin::{lexer, parser::abstract_syntax_tree::Root};

pub mod abstract_syntax_tree;

pub fn parse_source_code(code: &str) -> Result<Root, String> {
    let tokens = lexer::tokenize(code);

    Root::parse(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = r#"
            let x: int = 42;
            x = x + 1;
        "#;

        let result = parse_source_code(code);
        println!("{:#?}", result);
        assert!(result.is_ok());
        println!("{:#?}", result.unwrap());
    }
}