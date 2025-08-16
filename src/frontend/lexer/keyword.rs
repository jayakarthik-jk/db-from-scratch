use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Keyword {
    Create,
    Table,
    Alter,
    Drop,
    Insert,
    Update,
    Delete,
    Select,
    From,
    Int,
    Float,
    String,
    Boolean,
    DateTime,
    Add,
    Modify,
    Rename,
    Column,
    To,
    Into,
    Values,
    Where,
    In,
    Not,
    Like,
    And,
    Or,
    Set,
}

impl Keyword {
    pub(crate) fn get_keyword_kind(keyword: &str) -> Option<Keyword> {
        let keyword = match keyword.to_lowercase().as_str() {
            "create" => Keyword::Create,
            "table" => Keyword::Table,
            "alter" => Keyword::Alter,
            "drop" => Keyword::Drop,
            "insert" => Keyword::Insert,
            "update" => Keyword::Update,
            "delete" => Keyword::Delete,
            "select" => Keyword::Select,
            "from" => Keyword::From,
            "int" => Keyword::Int,
            "float" => Keyword::Float,
            "string" => Keyword::String,
            "boolean" => Keyword::Boolean,
            "datetime" => Keyword::DateTime,
            "add" => Keyword::Add,
            "modify" => Keyword::Modify,
            "rename" => Keyword::Rename,
            "column" => Keyword::Column,
            "to" => Keyword::To,
            "into" => Keyword::Into,
            "values" => Keyword::Values,
            "where" => Keyword::Where,
            "in" => Keyword::In,
            "not" => Keyword::Not,
            "like" => Keyword::Like,
            "and" => Keyword::And,
            "or" => Keyword::Or,
            "set" => Keyword::Set,
            _ => return None,
        };
        Some(keyword)
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword_str = match self {
            Keyword::Create => "CREATE",
            Keyword::Table => "TABLE",
            Keyword::Alter => "ALTER",
            Keyword::Drop => "DROP",
            Keyword::Insert => "INSERT",
            Keyword::Update => "UPDATE",
            Keyword::Delete => "DELETE",
            Keyword::Select => "SELECT",
            Keyword::From => "FROM",
            Keyword::Int => "INT",
            Keyword::Float => "FLOAT",
            Keyword::String => "STRING",
            Keyword::Boolean => "BOOLEAN",
            Keyword::DateTime => "DATETIME",
            Keyword::Add => "ADD",
            Keyword::Modify => "MODIFY",
            Keyword::Rename => "RENAME",
            Keyword::Column => "COLUMN",
            Keyword::To => "TO",
            Keyword::Into => "INTO",
            Keyword::Values => "VALUES",
            Keyword::Where => "WHERE",
            Keyword::In => "IN",
            Keyword::Not => "NOT",
            Keyword::Like => "LIKE",
            Keyword::And => "AND",
            Keyword::Or => "OR",
            Keyword::Set => "SET",
        };
        write!(f, "{keyword_str}")
    }
}
