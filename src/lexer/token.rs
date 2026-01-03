use std::fmt::Display;

use crate::{common::position::Span, lexer::literal::LiteralType};

use super::{keyword::Keyword, symbol::Symbol, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(LiteralType),
    Ident,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(k) => write!(f, "Keyword({})", k),
            TokenKind::Symbol(s) => write!(f, "Symbol({})", s),
            TokenKind::Literal(l) => write!(f, "Literal({})", l),
            TokenKind::Ident => write!(f, "Identifier"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn from_symbol(symbol: Symbol, position: Position) -> Self {
        Self {
            kind: TokenKind::Symbol(symbol),
            span: Span {
                start: position,
                end: position + (symbol.to_string().len() - 1usize),
            },
        }
    }
}
