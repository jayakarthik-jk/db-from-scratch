use std::fmt::Display;

use crate::{
    error::DBError,
    lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, Token},
    Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryOperator {
    // binary
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // comparision
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    // logical
    And,
    Or,

    // collection
    In,

    // string
    Like,
}

impl BinaryOperator {
    pub(crate) const fn match_symbol_with_precedence(
        symbol: &Symbol,
        precedence: u8,
    ) -> Option<Self> {
        let operator = match symbol {
            Symbol::Plus => BinaryOperator::Add,
            Symbol::Minus => BinaryOperator::Sub,
            Symbol::Star => BinaryOperator::Mul,
            Symbol::Divide => BinaryOperator::Div,
            Symbol::Carret => BinaryOperator::Pow,
            Symbol::Percent => BinaryOperator::Mod,
            // comparision
            Symbol::Equals => BinaryOperator::Equals,
            Symbol::NotEquals => BinaryOperator::NotEquals,
            Symbol::LessThan => BinaryOperator::LessThan,
            Symbol::LessThanOrEquals => BinaryOperator::LessThanOrEquals,
            Symbol::GreaterThan => BinaryOperator::GreaterThan,
            Symbol::GreaterThanOrEquals => BinaryOperator::GreaterThanOrEquals,
            _ => return None,
        };
        if precedence == operator.precedence() {
            return Some(operator);
        }
        None
    }

    pub(crate) fn parse_binary_operator<Tokens>(
        parser: &mut Parser<Tokens>,
        precedence: u8,
    ) -> Option<Result<Self, DBError>>
    where
        Tokens: Iterator<Item = Result<Token, DBError>>,
    {
        let token = match parser.tokens.peek()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err.to_owned())),
        };
        let operator = match token {
            Token {
                kind: TokenKind::Symbol(symbol),
                ..
            } => Self::match_symbol_with_precedence(symbol, precedence)?,
            Token {
                kind: TokenKind::Keyword(keyword),
                ..
            } => match keyword {
                Keyword::And if BinaryOperator::And.precedence() == precedence => {
                    BinaryOperator::And
                }
                Keyword::Or if BinaryOperator::Or.precedence() == precedence => BinaryOperator::Or,
                Keyword::In if BinaryOperator::In.precedence() == precedence => BinaryOperator::In,
                Keyword::Like if BinaryOperator::Like.precedence() == precedence => {
                    BinaryOperator::Like
                }
                _ => return None,
            },

            _ => return None,
        };

        assert!(parser.tokens.next().is_some());
        Some(Ok(operator))
    }

    pub(crate) const fn max_precedence() -> u8 {
        4
    }

    pub(crate) const fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::And => 5,
            BinaryOperator::Or => 5,
            BinaryOperator::In => 5,
            BinaryOperator::Like => 5,

            BinaryOperator::Equals => 4,
            BinaryOperator::NotEquals => 4,
            BinaryOperator::LessThan => 4,
            BinaryOperator::LessThanOrEquals => 4,
            BinaryOperator::GreaterThan => 4,
            BinaryOperator::GreaterThanOrEquals => 4,

            BinaryOperator::Pow => 3,

            BinaryOperator::Mul => 2,
            BinaryOperator::Div => 2,
            BinaryOperator::Mod => 2,

            BinaryOperator::Add => 1,
            BinaryOperator::Sub => 1,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => Symbol::Plus,
            BinaryOperator::Sub => Symbol::Minus,
            BinaryOperator::Mul => Symbol::Star,
            BinaryOperator::Div => Symbol::Divide,
            BinaryOperator::Mod => Symbol::Percent,
            BinaryOperator::Pow => Symbol::Carret,
            BinaryOperator::Equals => Symbol::Equals,
            BinaryOperator::NotEquals => Symbol::NotEquals,
            BinaryOperator::LessThan => Symbol::LessThan,
            BinaryOperator::LessThanOrEquals => Symbol::LessThanOrEquals,
            BinaryOperator::GreaterThan => Symbol::GreaterThan,
            BinaryOperator::GreaterThanOrEquals => Symbol::GreaterThanOrEquals,
            BinaryOperator::And => return write!(f, "{}", Keyword::And),
            BinaryOperator::Or => return write!(f, "{}", Keyword::Or),
            BinaryOperator::In => return write!(f, "{}", Keyword::In),
            BinaryOperator::Like => return write!(f, "{}", Keyword::Like),
        };
        write!(f, "{}", symbol)
    }
}
