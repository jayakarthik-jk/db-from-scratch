use crate::frontend::lexer::token::Identifier;

use super::statement::Column;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AlterType {
    Add(Column),
    Drop(Identifier),
    Modify(Column),
    Rename {
        old: Identifier,
        new: Identifier,
    },
}
