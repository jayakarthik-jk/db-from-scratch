use super::{keyword::Keyword, literal::Literal, symbol::Symbol, Position};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn from_keyword(keyword: Keyword, position: Position) -> Self {
        Self {
            kind: TokenKind::Keyword(keyword),
            span: Span {
                start: position,
                end: position + keyword.to_string().len(),
            },
        }
    }

    pub(crate) fn from_symbol(symbol: Symbol, position: Position) -> Self {
        Self {
            kind: TokenKind::Symbol(symbol),
            span: Span {
                start: position,
                end: position + symbol.to_string().len(),
            },
        }
    }
}

pub(crate) type Ident = String;
