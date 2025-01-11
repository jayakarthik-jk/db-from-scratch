use crate::frontend::lexer::{literal::Literal, symbol::Symbol, token::Identifier};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) enum Expression {
    Literal(Literal),
    Identifier(Identifier),
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
