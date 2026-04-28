use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl VariableValue {
    pub fn datatype(&self) -> VariableType {
        match self {
            VariableValue::Int(_) => VariableType::Int,
            VariableValue::Float(_) => VariableType::Float,
            VariableValue::Bool(_) => VariableType::Bool,
            VariableValue::String(_) => VariableType::String,
        }
    }

    pub fn is_type(&self, var_type: &VariableType) -> bool {
        self.datatype() == *var_type
    }

    pub fn parse_str(s: &str) -> Result<Self, String> {
        if s.starts_with("int(") && s.ends_with(")") {
            let inner = &s[4..s.len() - 1];
            match inner.parse::<i64>() {
                Ok(i) => Ok(VariableValue::Int(i)),
                Err(_) => Err(format!("Invalid integer value: {}", inner)),
            }
        } else if s.starts_with("float(") && s.ends_with(")") {
            let inner = &s[6..s.len() - 1];
            match inner.parse::<f64>() {
                Ok(f) => Ok(VariableValue::Float(f)),
                Err(_) => Err(format!("Invalid float value: {}", inner)),
            }
        } else if s.starts_with("bool(") && s.ends_with(")") {
            let inner = &s[5..s.len() - 1];
            match inner.parse::<bool>() {
                Ok(b) => Ok(VariableValue::Bool(b)),
                Err(_) => Err(format!("Invalid boolean value: {}", inner)),
            }
        } else if s.starts_with("string(") && s.ends_with(")") {
            let inner = &s[7..s.len() - 1];
            Ok(VariableValue::String(inner.to_string()))
        } else {
            Err(format!("Invalid VariableValue string: {}", s))
        }
    }
}

impl Display for VariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableValue::Int(i) => write!(f, "int({})", i),
            VariableValue::Float(fl) => write!(f, "float({})", fl),
            VariableValue::Bool(b) => write!(f, "bool({})", b),
            VariableValue::String(s) => write!(f, "string({})", s),
        }
    }
}

impl Serialize for VariableValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VariableValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VariableValue::parse_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Int,
    Float,
    Bool,
    String,
}

impl VariableType {
    pub fn parse_str(s: &str) -> Result<Self, String> {
        match s {
            "int" => Ok(VariableType::Int),
            "float" => Ok(VariableType::Float),
            "bool" => Ok(VariableType::Bool),
            "string" => Ok(VariableType::String),
            _ => Err(format!("Invalid VariableType string: {}", s)),
        }
    }
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Int => write!(f, "int"),
            VariableType::Float => write!(f, "float"),
            VariableType::Bool => write!(f, "bool"),
            VariableType::String => write!(f, "string"),
        }
    }
}

impl Serialize for VariableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VariableType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VariableType::parse_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
