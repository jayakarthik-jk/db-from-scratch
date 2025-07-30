use crate::frontend::lexer::{
    reader::Position, symbol::Symbol, token::TokenKind, LexerError, Token,
};

#[derive(Debug)]
pub(crate) struct ParserError {
    pub(crate) kind: ParserErrorKind,
    pub(crate) children: Vec<ParserErrorKind>,
}

pub(crate) trait AddError {
    fn add(&mut self, kind: ParserErrorKind);
    fn add_all(&mut self, ParserError { kind, children }: ParserError) {
        self.add(kind);
        for child in children {
            self.add(child);
        }
    }
}

impl AddError for ParserError {
    fn add(&mut self, kind: ParserErrorKind) {
        self.children.push(kind);
    }
}

impl AddError for Option<ParserError> {
    fn add(&mut self, kind: ParserErrorKind) {
        if let Some(error) = self {
            error.add(kind);
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        Self {
            kind: ParserErrorKind::LexerError(value),
            children: Vec::default(),
        }
    }
}

impl From<ParserErrorKind> for ParserError {
    fn from(value: ParserErrorKind) -> Self {
        Self {
            kind: value,
            children: Vec::default(),
        }
    }
}

#[derive(Debug)]
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
