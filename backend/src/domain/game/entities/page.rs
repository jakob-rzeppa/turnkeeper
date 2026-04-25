use crate::domain::common::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    id: Identifier,
    name: String,

    code: String,
    starting_line_number: usize,
}

impl Page {
    pub fn new(name: String, code: String, starting_line_number: usize) -> Self {
        Self {
            id: Identifier::new(),
            name,
            code,
            starting_line_number,
        }
    }

    pub fn new_raw(
        id: Identifier,
        name: String,
        code: String,
        starting_line_number: usize,
    ) -> Self {
        Self {
            id,
            name,
            code,
            starting_line_number,
        }
    }
}
