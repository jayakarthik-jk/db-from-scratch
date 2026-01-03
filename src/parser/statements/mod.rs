pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod insert;
pub(crate) mod select;
pub(crate) mod update;

use super::{datatype::Datatype, expression::Expression};
use crate::{common::position::Span, error::DBError, lexer::{Token, keyword::Keyword, token::TokenKind}, parser::Parser};
use update::UpdateSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Column {
    pub name: Span,
    pub data_type: Datatype,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    CreateDatabase {
        database_name: Span,
    },
    // DDL
    CreateTable {
        table_name: Span,
        columns: Vec<Column>,
    },
    DropTable {
        table_name: Span,
    },
    // DML
    Insert {
        table_name: Span,
        columns: Option<Vec<Span>>,
        values: Vec<Expression>,
    },
    Update {
        table_name: Span,
        set: Vec<UpdateSet>,
        predicate: Option<Expression>,
    },
    Delete {
        table_name: Span,
        predicate: Option<Expression>,
    },
    // DQL
    Select {
        select_expressions: Vec<Expression>,
        from: Option<Span>,
        predicate: Option<Expression>,
    },
}


impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_drop_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Table))?;
        let table_name = self.expect_identifier()?;
        Ok(Statement::DropTable { table_name })
    }
}
