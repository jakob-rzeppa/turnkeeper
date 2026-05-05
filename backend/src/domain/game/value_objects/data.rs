use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn datatype(&self) -> Datatype {
        match self {
            Value::Int(_) => Datatype::Int,
            Value::Float(_) => Datatype::Float,
            Value::Bool(_) => Datatype::Bool,
            Value::String(_) => Datatype::String,
        }
    }

    pub fn is_type(&self, var_type: &Datatype) -> bool {
        self.datatype() == *var_type
    }

    pub fn parse_str(s: &str) -> Result<Self, String> {
        if s.starts_with("int(") && s.ends_with(")") {
            let inner = &s[4..s.len() - 1];
            match inner.parse::<i64>() {
                Ok(i) => Ok(Value::Int(i)),
                Err(_) => Err(format!("Invalid integer value: {}", inner)),
            }
        } else if s.starts_with("float(") && s.ends_with(")") {
            let inner = &s[6..s.len() - 1];
            match inner.parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err(format!("Invalid float value: {}", inner)),
            }
        } else if s.starts_with("bool(") && s.ends_with(")") {
            let inner = &s[5..s.len() - 1];
            match inner.parse::<bool>() {
                Ok(b) => Ok(Value::Bool(b)),
                Err(_) => Err(format!("Invalid boolean value: {}", inner)),
            }
        } else if s.starts_with("string(") && s.ends_with(")") {
            let inner = &s[7..s.len() - 1];
            Ok(Value::String(inner.to_string()))
        } else {
            Err(format!("Invalid VariableValue string: {}", s))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "int({})", i),
            Value::Float(fl) => write!(f, "float({})", fl),
            Value::Bool(b) => write!(f, "bool({})", b),
            Value::String(s) => write!(f, "string({})", s),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Value::parse_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Datatype {
    Int,
    Float,
    Bool,
    String,
}

impl Datatype {
    pub fn parse_str(s: &str) -> Result<Self, String> {
        match s {
            "int" => Ok(Datatype::Int),
            "float" => Ok(Datatype::Float),
            "bool" => Ok(Datatype::Bool),
            "string" => Ok(Datatype::String),
            _ => Err(format!("Invalid VariableType string: {}", s)),
        }
    }
}

impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datatype::Int => write!(f, "int"),
            Datatype::Float => write!(f, "float"),
            Datatype::Bool => write!(f, "bool"),
            Datatype::String => write!(f, "string"),
        }
    }
}

impl Serialize for Datatype {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Datatype {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Datatype::parse_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
