use serde::Serialize;

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

    pub fn parse_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid parameter format: {}", s));
        }
        let name = parts[0].trim().to_string();
        let datatype_str = parts[1].trim();
        let datatype = Datatype::parse_str(datatype_str)?;
        Ok(Parameter { name, datatype })
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.datatype)
    }
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Parameter::parse_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
