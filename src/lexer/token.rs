use crate::{common::position::Span, lexer::literal::Literal};

use super::{keyword::Keyword, symbol::Symbol, Position};

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

pub(crate) type Ident = String;
