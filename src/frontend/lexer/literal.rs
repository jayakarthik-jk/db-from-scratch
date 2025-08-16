use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Integer(value) => value.to_string(),
                Literal::Float(value) => value.to_string(),
                Literal::Boolean(value) => value.to_string(),
                Literal::String(value) => value.to_string(),
            }
        )
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        return Self::String(value.to_string());
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        return Self::String(value);
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        return Self::Integer(value);
    }
}
impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        return Self::Boolean(value);
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        return Self::Float(value);
    }
}

impl Literal {
    pub(crate) fn get_literal(keyword: &str) -> Option<Literal> {
        let keyword = match keyword.to_lowercase().as_str() {
            "true" => Literal::Boolean(true),
            "false" => Literal::Boolean(false),
            _ => return None,
        };
        return Some(keyword);
    }
}
