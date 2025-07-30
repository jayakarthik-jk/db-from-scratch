#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Keyword {
    Select,
    Insert,
    From,
    //Update,
    //Delete,
}

impl Keyword {
    pub(crate) fn get_keyword_kind(keyword: &str) -> Option<Keyword> {
        let keyword = match keyword.to_lowercase().as_str() {
            "select" => Keyword::Select,
            "insert" => Keyword::Insert,
            "from" => Keyword::From,
            //"delete" => Keyword::Delete,
            //"update" => Keyword::Update,
            _ => return None,
        };
        return Some(keyword);
    }
}
