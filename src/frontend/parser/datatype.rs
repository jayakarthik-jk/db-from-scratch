use std::fmt::Display;

use crate::frontend::lexer::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Datatype {
    Integer,
    Float,
    String,
    Boolean,
    DateTime,
}

impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datatype::Integer => write!(f, "INTEGER"),
            Datatype::Float => write!(f, "FLOAT"),
            Datatype::String => write!(f, "STRING"),
            Datatype::Boolean => write!(f, "BOOLEAN"),
            Datatype::DateTime => write!(f, "DATETIME"),
        }
    }
}

impl Datatype {
    pub(crate) fn from_keyword(keyword: Keyword) -> Option<Datatype> {
        Some(match keyword {
            Keyword::Int => Datatype::Integer,
            Keyword::Float => Datatype::Float,
            Keyword::String => Datatype::String,
            Keyword::Boolean => Datatype::Boolean,
            Keyword::DateTime => Datatype::DateTime,
            _ => return None, // Only these keywords are valid for types
        })
    }
}
