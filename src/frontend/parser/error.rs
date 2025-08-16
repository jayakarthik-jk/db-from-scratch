use crate::frontend::lexer::{keyword::Keyword, token::TokenKind, LexerError, Token};

#[derive(Debug, PartialEq)]
pub(crate) enum ParserError {
    NotAnExpression,
    LexerError(LexerError),
    Unexpected { found: Token, expected: TokenKind },
    IdentExpected(Token),
    KeywordExpected(Token),
    DatatypeExpected(Token),
    UnexpectedStatement,
    UnExpectedAlterType { expected: Keyword, found: TokenKind },
    Custom(String),
    Eof,
}

pub trait IntoParseResult<T> {
    fn as_err(&self) -> Result<T, ParserError>;
}

impl<T> IntoParseResult<T> for &str {
    fn as_err(&self) -> Result<T, ParserError> {
        Err(ParserError::Custom(self.to_string()))
    }
}
