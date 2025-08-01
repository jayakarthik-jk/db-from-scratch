use std::fmt::Display;

use crate::frontend::lexer::{literal::Literal, symbol::Symbol, token::Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryOperator {
    // binary
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    // comparision
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl BinaryOperator {
    pub(crate) fn match_symbol_with_precedence(symbol: Symbol, precedence: u8) -> Option<Self> {
        let operator = match symbol {
            Symbol::Plus => BinaryOperator::Addition,
            Symbol::Minus => BinaryOperator::Subtraction,
            Symbol::Star => BinaryOperator::Multiplication,
            Symbol::Divide => BinaryOperator::Division,
            Symbol::Percent => BinaryOperator::Modulo,
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

    pub(crate) fn max_precedence() -> u8 {
        3
    }

    pub(crate) fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Equals => 3,
            BinaryOperator::NotEquals => 3,
            BinaryOperator::LessThan => 3,
            BinaryOperator::LessThanOrEquals => 3,
            BinaryOperator::GreaterThan => 3,
            BinaryOperator::GreaterThanOrEquals => 3,

            BinaryOperator::Multiplication => 2,
            BinaryOperator::Division => 2,
            BinaryOperator::Modulo => 2,

            BinaryOperator::Addition => 1,
            BinaryOperator::Subtraction => 1,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            BinaryOperator::Addition => Symbol::Plus,
            BinaryOperator::Subtraction => Symbol::Minus,
            BinaryOperator::Multiplication => Symbol::Star,
            BinaryOperator::Division => Symbol::Divide,
            BinaryOperator::Modulo => Symbol::Percent,
            BinaryOperator::Equals => Symbol::Equals,
            BinaryOperator::NotEquals => Symbol::NotEquals,
            BinaryOperator::LessThan => Symbol::LessThan,
            BinaryOperator::LessThanOrEquals => Symbol::LessThanOrEquals,
            BinaryOperator::GreaterThan => Symbol::GreaterThan,
            BinaryOperator::GreaterThanOrEquals => Symbol::GreaterThanOrEquals,
        };
        write!(f, "{}", symbol)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssignmentOperator {
    Equal,
    PlusEquals,
    MinusEquals,
    StarEquals,
    DivideEquals,
    PercentEquals,
}

impl AssignmentOperator {
    pub(crate) fn match_symbol(symbol: Symbol) -> Option<Self> {
        let operator = match symbol {
            Symbol::Equal => AssignmentOperator::Equal,
            Symbol::PlusEquals => AssignmentOperator::PlusEquals,
            Symbol::MinusEquals => AssignmentOperator::MinusEquals,
            Symbol::StarEquals => AssignmentOperator::StarEquals,
            Symbol::DivideEquals => AssignmentOperator::DivideEquals,
            Symbol::PercentEquals => AssignmentOperator::PercentEquals,
            _ => return None,
        };
        Some(operator)
    }
}

impl Display for AssignmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            AssignmentOperator::Equal => Symbol::Equal,
            AssignmentOperator::PlusEquals => Symbol::PlusEquals,
            AssignmentOperator::MinusEquals => Symbol::MinusEquals,
            AssignmentOperator::StarEquals => Symbol::StarEquals,
            AssignmentOperator::DivideEquals => Symbol::DivideEquals,
            AssignmentOperator::PercentEquals => Symbol::PercentEquals,
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Wildcard,
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall {
        ident: Identifier,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Assign {
        ident: Identifier,
        operator: AssignmentOperator,
        right: Box<Expression>,
    },
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
            Expression::Binary { left, operator, right } => {
                write!(f, "{} {} {}", left, operator, right)
            }
            Expression::Assign { ident, operator, right } => {
                write!(f, "{} {} {}", ident, operator, right)
            }
        }
    }
}
