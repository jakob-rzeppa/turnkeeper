use crate::domain::common::identifier::Identifier;

pub struct Action {
    id: Identifier,
    name: String,

    code: String,
    starting_line_number: usize,
}
