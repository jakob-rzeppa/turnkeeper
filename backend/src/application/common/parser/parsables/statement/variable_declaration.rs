use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable, parsables::expression::Expression}, domain::{common::position::{Position, Positioned}, game::value_objects::data::Datatype}};



#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationStatement {
    name: String,
    var_type: Datatype,
    value: Expression,
    pos: Position,
}

impl VariableDeclarationStatement {
    pub fn name(&self) -> &str {
        &self.name
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

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::Let,
            "Expected 'let' keyword at the beginning of variable declaration"
        );

        let name = match ts.next() {
            Some(t) if matches!(t.variant, TokenVariant::Identifier(_)) => {
                if let TokenVariant::Identifier(name) = &t.variant {
                    name.clone()
                } else {
                    unreachable!("Token wasn't an Identifier after checking it was one.")
                }
            }
            Some(t) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected identifier after 'let' in variable declaration".to_string(),
                    found: t.variant.clone(),
                    pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected identifier after 'let' in variable declaration".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Colon,
            "':' after identifier in variable declaration"
        );

        let var_type = Datatype::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected variable type (int, float, string, bool) in variable declaration")
        )?;

        expect_token!(
            ts,
            TokenVariant::Equal,
            "Expected '=' after datatype in variable declaration"
        );

        let value = Expression::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression after '=' in variable declaration")
        )?;

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "Expected ';' at the end of variable declaration"
        );

        Ok(VariableDeclarationStatement {
            name,
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
        name: &str,
        var_type: Datatype,
        value: Expression,
        line: usize,
        first_char: usize,
    ) -> Self {
        VariableDeclarationStatement {
            name: name.to_string(),
            var_type,
            value,
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::{macros::test_token_stream, parsables::expression::binary::BinaryOperator};

use super::*;

    #[test]
    fn test_variable_declaration_int_parsing() {
        let (mut ts, source_code) = test_token_stream!("let x: int = 42;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "x",
                Datatype::Int,
                Expression::new_atom_literal_int(42, 0, 13),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_float_parsing() {
        let (mut ts, source_code) = test_token_stream!("let pi: float = 3.14;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "pi",
                Datatype::Float,
                Expression::new_atom_literal_float(3.14, 0, 16),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_string_parsing() {
        let (mut ts, source_code) = test_token_stream!("let name: string = \"Hello\";");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "name",
                Datatype::String,
                Expression::new_atom_literal_string("Hello".to_string(), 0, 19),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_bool_parsing() {
        let (mut ts, source_code) = test_token_stream!("let flag: bool = true;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "flag",
                Datatype::Bool,
                Expression::new_atom_literal_bool(true, 0, 17),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_with_expression_parsing() {
        let (mut ts, source_code) = test_token_stream!("let sum: int = 1 + 2;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "sum",
                Datatype::Int,
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 15),
                    BinaryOperator::Addition,
                    Expression::new_atom_literal_int(2, 0, 19),
                    0,
                    15
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_variable_declaration_not_next_on_non_let() {
        let (ts, _source_code) = test_token_stream!("x");

        assert!(!VariableDeclarationStatement::is_next(&ts));
    }
}
