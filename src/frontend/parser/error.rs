use crate::frontend::lexer::{keyword::Keyword, token::TokenKind, LexerError, Token};

#[derive(Debug, PartialEq)]
pub(crate) struct ParserError {
    pub(crate) kind: ParserErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParserErrorKind {
    NotAnExpression,
    LexerError(LexerError),
    // UnexpectedWithExpected {
    //     expected: TokenKind,
    //     found: Token,
    // },
    Unexpected(Token),
    UnexpectedStatement,
    UnExpectedAlterType { expected: Keyword, found: TokenKind },
    Custom(String),
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

pub trait IntoParseResult<T> {
    fn as_err(self) -> Option<Result<T, ParserError>>;
}

impl<T> IntoParseResult<T> for &str {
    fn as_err(self) -> Option<Result<T, ParserError>> {
        Some(Err(ParserErrorKind::Custom(self.to_string()).into()))
    }
}
