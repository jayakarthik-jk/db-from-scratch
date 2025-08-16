use std::fmt::Display;

use crate::frontend::lexer::{keyword::Keyword, token::TokenKind, LexerError, Token};

#[derive(Debug, PartialEq)]
pub(crate) enum ParserError {
    NotAnExpression,
    LexerError(LexerError),
    Unexpected { found: Token, expected: TokenKind },
    UnexpectedToken { expected: TokenKind },
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

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // LINE: {} COLUMN: {} - {message}
        match self {
            ParserError::NotAnExpression => write!(f, "Not an expression"),
            ParserError::LexerError(err) => write!(f, "{}", err),
            ParserError::Unexpected { found, expected } => {
                write!(f, "Unexpected token: found {:?}, expected {:?}", found, expected)
            }
            ParserError::UnexpectedToken { expected } => {
                write!(f, "Unexpected token, expected {:?}", expected)
            }
            ParserError::IdentExpected(token) => {
                write!(f, "Identifier expected, found {:?}", token)
            }
            ParserError::KeywordExpected(token) => {
                write!(f, "Keyword expected, found {:?}", token)
            }
            ParserError::DatatypeExpected(token) => {
                write!(f, "Data type expected, found {:?}", token)
            }
            ParserError::UnexpectedStatement => write!(f, "Unexpected statement"),
            ParserError::UnExpectedAlterType { expected, found } => {
                write!(f, "Expected {:?}, found {:?}", expected, found)
            }
            ParserError::Custom(message) => write!(f, "Custom error: {}", message),
            ParserError::Eof => write!(f, "End of file reached unexpectedly"),
        }
    }
}
