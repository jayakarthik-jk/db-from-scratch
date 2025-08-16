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
    pub(crate) fn parse_drop_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Table))?;
        let table_name = self.expected_identifier()?;
        Ok(Statement::Drop { table_name })
    }
}
