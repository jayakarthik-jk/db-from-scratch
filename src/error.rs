use std::fmt::Display;

use crate::lexer::{keyword::Keyword, reader::Position, token::TokenKind, Token};

#[derive(Debug, Clone)]
pub(crate) enum DBError {
    // Lexer errors
    UnTerminatedStringLiteral(Position),
    NumberExceededSize(Position),
    IllegalCharacter(char, Position),

    // Parser errors
    Unexpected { found: Token, expected: TokenKind },
    UnexpectedEof { expected: TokenKind },
    UnexpectedToken { found: Token },
    IdentExpected(Token),
    KeywordExpected(Token),
    DatatypeExpected(Token),
    UnexpectedStatement,
    UnExpectedAlterType { expected: Keyword, found: TokenKind },
    Custom(String),
    Eof,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::UnTerminatedStringLiteral(pos) => {
                write!(f, "Unterminated string literal at position: {:?}", pos)
            }
            DBError::NumberExceededSize(pos) => {
                write!(f, "Number exceeded size limit at position: {:?}", pos)
            }
            DBError::IllegalCharacter(ch, pos) => {
                write!(f, "Illegal character '{}' at position: {:?}", ch, pos)
            }


            DBError::Unexpected { found, expected } => {
                write!(
                    f,
                    "Unexpected token: found {:?}, expected {:?}",
                    found, expected
                )
            }
            DBError::UnexpectedEof { expected } => {
                write!(f, "Unexpected end of file, expected {:?}", expected)
            }
            DBError::UnexpectedToken { found } => {
                write!(f, "Unexpected token: {:?}", found)
            }
            DBError::IdentExpected(token) => {
                write!(f, "Identifier expected, found {:?}", token)
            }
            DBError::KeywordExpected(token) => {
                write!(f, "Keyword expected, found {:?}", token)
            }
            DBError::DatatypeExpected(token) => {
                write!(f, "Data type expected, found {:?}", token)
            }
            DBError::UnexpectedStatement => write!(f, "Unexpected statement"),
            DBError::UnExpectedAlterType { expected, found } => {
                write!(f, "Expected {:?}, found {:?}", expected, found)
            }
            DBError::Custom(message) => write!(f, "Custom error: {}", message),
            DBError::Eof => write!(f, "End of file reached unexpectedly"),
        }
    }
}

pub trait IntoParseResult<T> {
    fn as_err(&self) -> Result<T, DBError>;
}

impl<T> IntoParseResult<T> for &str {
    fn as_err(&self) -> Result<T, DBError> {
        Err(DBError::Custom(self.to_string()))
    }
}
