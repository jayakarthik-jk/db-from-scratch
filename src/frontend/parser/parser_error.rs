use crate::frontend::lexer::{
    reader::Position, symbol::Symbol, token::TokenKind, LexerError, Token,
};

#[derive(Debug, PartialEq)]
pub(crate) struct ParserError {
    pub(crate) kind: ParserErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParserErrorKind {
    NotAnExpression,
    LexerError(LexerError),
    TrailingSeperator {
        position: Position,
        seperator: Symbol,
    },
    MissingSeperator {
        //position: Position,
        seperator: Symbol,
    },
    KeywordExpected(Token),
    UnexpectedToken {
        expected: TokenKind,
        found: Token,
    },
    Unexpected(Token),
    IncompleteExpression(Token),
    UnexpectedStatement,
    TableNameExpected(Token),
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        Self {
            kind: ParserErrorKind::LexerError(value),
        }
    }
}

impl From<ParserErrorKind> for ParserError {
    fn from(value: ParserErrorKind) -> Self {
        Self { kind: value }
    }
}
