use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Symbol {
    OpenParanthesis,
    CloseParanthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,

    Comma,
    // Colon,
    Semicolon,

    // binary
    Plus,
    Minus,
    Star,
    Divide,
    Percent,

    // assignment
    Equal,
    PlusEquals,
    MinusEquals,
    StarEquals,
    DivideEquals,
    PercentEquals,

    // comparision
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    // logical
    And,
    Or,
    Not,

    // bitwise
    BitAnd,
    BitOr,
    BitNot,
    BitXor,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Symbol::OpenParanthesis => "(",
            Symbol::CloseParanthesis => ")",
            Symbol::OpenSquareBracket => "[",
            Symbol::CloseSquareBracket => "]",
            Symbol::OpenCurlyBracket => "{",
            Symbol::CloseCurlyBracket => "}",

            Symbol::Comma => ",",
            Symbol::Semicolon => ";",

            // conditional
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Star => "*",
            Symbol::Divide => "/",
            Symbol::Percent => "%",
            // assignment
            Symbol::Equal => "=",
            Symbol::PlusEquals => "+=",
            Symbol::MinusEquals => "-=",
            Symbol::StarEquals => "*=",
            Symbol::DivideEquals => "/=",
            Symbol::PercentEquals => "%=",
            // comparision
            Symbol::Equals => "==",
            Symbol::NotEquals => "!=",
            Symbol::GreaterThanOrEquals => ">=",
            Symbol::GreaterThan => ">",
            Symbol::LessThanOrEquals => "<=",
            Symbol::LessThan => "<",
            // logical
            Symbol::And => "&&",
            Symbol::Or => "||",
            Symbol::Not => "!",
            // bitwise
            Symbol::BitAnd => "&",
            Symbol::BitOr => "|",
            Symbol::BitNot => "~",
            Symbol::BitXor => "^",
        };
        write!(f, "{}", text)
    }
}
