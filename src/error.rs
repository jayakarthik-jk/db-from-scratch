use crate::{
    common::position::{Position, Span},
    lexer::{keyword::Keyword, token::TokenKind, Token},
    source::RawStatement,
};

#[derive(Debug, Clone)]
pub(crate) enum DBError {
    // Lexer errors
    UnTerminatedString(Span),
    UnterminatedFloat(Span),
    IllegalCharacter(char, Position),

    // Parser errors
    Unexpected {
        found: Token,
        expected: TokenKind,
    },
    UnexpectedEof {
        expected: TokenKind,
    },
    UnexpectedToken {
        found: Token,
    },
    UnexpectedKeyword {
        found: Keyword,
        allowed: Vec<Keyword>,
    },
    IdentExpected(Token),
    KeywordExpected(Token),
    DatatypeExpected(Token),
    UnexpectedStatement,
    Eof,
}

impl DBError {
    pub(crate) fn print(&self, rs: &RawStatement) {
        use DBError::*;

        let message = match self {
            IdentExpected(token) => format!("Identifier expected, found {}", rs.slice(&token.span)),
            KeywordExpected(token) => format!("Keyword expected, found {}", rs.slice(&token.span)),
            DatatypeExpected(token) => {
                format!("Datatype expected, found {}", rs.slice(&token.span))
            }
            Unexpected { found, expected } => format!(
                "Unexpected token: found {}, expected {}",
                rs.slice(&found.span),
                expected
            ),
            UnexpectedEof { expected } => format!("Unexpected end of file: expected {}", expected),
            UnexpectedToken { found } => {
                format!("Unexpected token: found {}", rs.slice(&found.span))
            }
            UnTerminatedString(pos) => {
                format!("Unterminated string at {}", pos.start)
            }
            UnterminatedFloat(span) => format!("Unterminated Float '{}'", rs.slice(span)),
            IllegalCharacter(c, pos) => format!("Illegal character '{}' at {}", c, pos),
            UnexpectedStatement => "Unexpected statement".to_string(),
            Eof => "End of file reached unexpectedly".to_string(),
            UnexpectedKeyword { found, allowed } => {
                let allowed_keywords: Vec<String> = allowed.iter().map(|k| k.to_string()).collect();
                format!(
                    "Unexpected keyword '{}'. Allowed keywords are: {}",
                    found,
                    allowed_keywords.join(", ")
                )
            }
        };

        eprintln!("Error: {}", message);
    }
}
