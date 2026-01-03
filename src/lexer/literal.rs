use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LiteralType {
    Boolean,
    Integer,
    Float,
    String,
}

impl LiteralType {
    pub(crate) fn get_literal(keyword: &str) -> Option<LiteralType> {
        let keyword = match keyword.to_lowercase().as_str() {
            "true" | "false" => LiteralType::Boolean,
            _ => return None,
        };
        Some(keyword)
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::Boolean => write!(f, "Boolean"),
            LiteralType::Integer => write!(f, "Integer"),
            LiteralType::Float => write!(f, "Float"),
            LiteralType::String => write!(f, "String"),
        }
    }
}
