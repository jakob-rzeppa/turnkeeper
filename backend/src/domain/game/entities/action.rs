use crate::domain::common::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    id: Identifier,
    name: String,

    code: String,
    starting_line_number: usize,
}

impl Action {
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

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn starting_line_number(&self) -> usize {
        self.starting_line_number
    }
}
