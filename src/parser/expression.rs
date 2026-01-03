use super::operators::binary::BinaryOperator;
use crate::{common::position::Span, lexer::literal::LiteralType};

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Literal(LiteralType, Span),
    Ident(Span),
    FunctionCall {
        name: Span,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Negation(Box<Expression>),
}
