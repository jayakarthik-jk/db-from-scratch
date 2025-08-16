use crate::{
    frontend::{
        lexer::{
            keyword::Keyword,
            symbol::Symbol,
            token::{Ident, TokenKind},
            LexerError, Token,
        },
        parser::{error::ParserError, expression::Expression},
    },
    util::layer::Layer,
    Parser,
};

use super::Statement;

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateSet {
    pub(crate) column: Ident,
    pub(crate) value: Expression,
}

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_update_statement(&mut self) -> Result<Statement, ParserError> {
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

    pub(crate) fn parse_update_set(&mut self) -> Result<UpdateSet, ParserError> {
        let column = self.expect_ident()?;

        self.expect(TokenKind::Symbol(Symbol::Equal))?;

        let value = self.parse_expression()?;

        Ok(UpdateSet { column, value })
    }
}
