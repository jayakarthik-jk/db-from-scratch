use super::Statement;
use crate::{
    common::layer::Layer,
    error::DBError,
    lexer::{keyword::Keyword, token::TokenKind, Token},
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, DBError>,
{
    pub(crate) fn parse_delete_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::From))?;
        let table_name = self.expected_identifier()?;
        let predicate = self.parse_predicate()?;

        Ok(Statement::Delete {
            table_name,
            predicate,
        })
    }
}
