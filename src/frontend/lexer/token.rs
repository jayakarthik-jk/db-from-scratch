use super::{keyword::Keyword, literal::Literal, symbol::Symbol, Position};

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) position: Position,
}
impl Token {
    pub(crate) fn new(kind: TokenKind, position: Position) -> Self {
        Self { kind, position }
    }

    pub(crate) fn from_symbol(symbol: Symbol, position: Position) -> Self {
        Self {
            kind: TokenKind::Symbol(symbol),
            position,
        }
    }
}

pub(crate) type Identifier = String;
