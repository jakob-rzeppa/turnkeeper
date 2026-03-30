use crate::application::game::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{
            Parsable, Positioned, TokenStream,
            atom::{datatype::Datatype, identifier::Identifier},
            expression::Expression,
        },
        error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationStatement {
    identifier: Identifier,
    var_type: Datatype,
    value: Expression,
    pos: Position,
}

impl VariableDeclarationStatement {
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn datatype(&self) -> &Datatype {
        &self.var_type
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Parsable for VariableDeclarationStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Let)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::Let,
            "'let' keyword at the beginning of variable declaration"
        );

        let identifier = expect_parse!(
            ts,
            Identifier,
            "identifier after 'let' keyword in variable declaration"
        );

        expect_token!(
            ts,
            TokenVariant::Colon,
            "':' after identifier in variable declaration"
        );

        let var_type = expect_parse!(ts, Datatype, "datatype after ':' in variable declaration");

        expect_token!(
            ts,
            TokenVariant::Assign,
            "'=' after datatype in variable declaration"
        );

        let value = expect_parse!(
            ts,
            Expression,
            "expression after '=' in variable declaration"
        );

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "';' at the end of variable declaration"
        );

        Ok(VariableDeclarationStatement {
            identifier,
            var_type,
            value,
            pos,
        })
    }
}

impl Positioned for VariableDeclarationStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl VariableDeclarationStatement {
    pub fn new(
        identifier: &str,
        var_type: Datatype,
        value: Expression,
        line: usize,
        first_char: usize,
    ) -> Self {
        VariableDeclarationStatement {
            identifier: Identifier::new(identifier),
            var_type,
            value,
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration_int_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Let,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::EqualEqual,
            TokenVariant::IntLiteral(42),
            TokenVariant::Semicolon
        );

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "x",
                Datatype::Integer,
                Expression::new_atom_literal_int(42, 5, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_float_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Let,
            TokenVariant::Identifier("pi".to_string()),
            TokenVariant::Colon,
            TokenVariant::FloatType,
            TokenVariant::EqualEqual,
            TokenVariant::FloatLiteral(3.14),
            TokenVariant::Semicolon
        );

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "pi",
                Datatype::Float,
                Expression::new_atom_literal_float(3.14, 5, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_string_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Let,
            TokenVariant::Identifier("name".to_string()),
            TokenVariant::Colon,
            TokenVariant::StringType,
            TokenVariant::EqualEqual,
            TokenVariant::StringLiteral("Hello".to_string()),
            TokenVariant::Semicolon
        );

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "name",
                Datatype::String,
                Expression::new_atom_literal_string("Hello".to_string(), 5, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_bool_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Let,
            TokenVariant::Identifier("flag".to_string()),
            TokenVariant::Colon,
            TokenVariant::BoolType,
            TokenVariant::EqualEqual,
            TokenVariant::BoolLiteral(true),
            TokenVariant::Semicolon
        );

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "flag",
                Datatype::Boolean,
                Expression::new_atom_literal_bool(true, 5, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_with_expression_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Let,
            TokenVariant::Identifier("sum".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::EqualEqual,
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2),
            TokenVariant::Semicolon
        );

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "sum",
                Datatype::Integer,
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 5, 0),
                    crate::application::game::plugin::parser::abstract_syntax_tree::expression::binary::BinaryOperator::Addition,
                    Expression::new_atom_literal_int(2, 7, 0),
                    5,
                    0
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_not_next_on_non_let() {
        let ts = test_token_stream!(TokenVariant::Identifier("x".to_string()));

        assert!(!VariableDeclarationStatement::is_next(&ts));
    }
}
