use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
        if s.starts_with("Int(") && s.ends_with(")") {
            let inner = &s[4..s.len() - 1];
            match inner.parse::<i64>() {
                Ok(i) => Ok(VariableValue::Int(i)),
                Err(_) => Err(format!("Invalid integer value: {}", inner)),
            }
        } else if s.starts_with("Float(") && s.ends_with(")") {
            let inner = &s[6..s.len() - 1];
            match inner.parse::<f64>() {
                Ok(f) => Ok(VariableValue::Float(f)),
                Err(_) => Err(format!("Invalid float value: {}", inner)),
            }
        } else if s.starts_with("Bool(") && s.ends_with(")") {
            let inner = &s[5..s.len() - 1];
            match inner.parse::<bool>() {
                Ok(b) => Ok(VariableValue::Bool(b)),
                Err(_) => Err(format!("Invalid boolean value: {}", inner)),
            }
        } else if s.starts_with("String(") && s.ends_with(")") {
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
            VariableValue::Int(i) => write!(f, "Int({})", i),
            VariableValue::Float(fl) => write!(f, "Float({})", fl),
            VariableValue::Bool(b) => write!(f, "Bool({})", b),
            VariableValue::String(s) => write!(f, "String({})", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VariableType {
    Int,
    Float,
    Bool,
    String,
}
