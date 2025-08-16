use super::operators::binary::BinaryOperator;
use crate::frontend::lexer::{literal::Literal, token::Ident};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Wildcard,
    Literal(Literal),
    Identifier(Ident),
    FunctionCall {
        ident: Ident,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Negation(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Wildcard => write!(f, "*"),
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::FunctionCall { ident, arguments } => {
                write!(f, "{}(", ident)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "{} {} {}", left, operator, right)
            }
            Expression::Negation(expr) => {
                write!(f, "NOT ({})", expr)
            }
        }
    }
}
