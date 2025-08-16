use crate::{
    error::DBError,
    lexer::{
        keyword::Keyword,
        symbol::Symbol,
        token::{Ident, TokenKind},
        Token,
    },
    parser::expression::Expression,
    Parser,
};

use super::Statement;

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateSet {
    pub(crate) column: Ident,
    pub(crate) value: Expression,
}

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_update_statement(&mut self) -> Result<Statement, DBError> {
        let table_name = self.expect_ident()?;

        self.expect(TokenKind::Keyword(Keyword::Set))?;

        let update_set = self.parse_seperated(Symbol::Comma, |parser| parser.parse_update_set())?;

        let predicate = self.parse_predicate()?;

        Ok(Statement::Update {
            table_name,
            set: update_set,
            predicate,
        })
    }

    pub(crate) fn parse_update_set(&mut self) -> Result<UpdateSet, DBError> {
        let column = self.expect_ident()?;

        self.expect(TokenKind::Symbol(Symbol::Equal))?;

        let value = self.parse_expression()?;

        Ok(UpdateSet { column, value })
    }
}
