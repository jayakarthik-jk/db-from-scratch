use crate::frontend::lexer::token::Identifier;

use super::expression::Expression;

#[derive(Debug)]
pub(crate) enum Statement {
    // Base Statement
    Insert,
    Select(SelectStatement),
}

#[derive(Debug)]
pub(crate) struct SelectStatement {
    pub(crate) select_expressions: Vec<Expression>,
    pub(crate) from: Option<Identifier>,
}
