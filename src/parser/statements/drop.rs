use super::Statement;
use crate::{
    common::layer::Layer,
    Parser,
    {
        lexer::{keyword::Keyword, token::TokenKind, LexerError, Token},
        parser::error::ParserError,
    },
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_drop_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenKind::Keyword(Keyword::Table))?;
        let table_name = self.expected_identifier()?;
        Ok(Statement::Drop { table_name })
    }
}
