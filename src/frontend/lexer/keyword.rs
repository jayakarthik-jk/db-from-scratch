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
            _ => return None,
        };
        return Some(keyword);
    }
}
