use std::{fmt::Display, str::FromStr};

use backend_derive::{deserialize_use_from_str, serialize_use_display};

use crate::domain::game::value_objects::data::Datatype;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    name: String,
    datatype: Datatype,
}

impl Parameter {
    pub fn new(name: String, datatype: Datatype) -> Self {
        Self { name, datatype }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }
}

#[serialize_use_display]
impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.datatype)
    }
}

#[deserialize_use_from_str]
impl FromStr for Parameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid parameter format: {}", s));
        }
        let name = parts[0].trim().to_string();
        let datatype_str = parts[1].trim();
        let datatype = Datatype::from_str(datatype_str)?;
        Ok(Parameter { name, datatype })
    }
}