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
    pub(crate) fn parse_delete_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenKind::Keyword(Keyword::From))?;
        let table_name = self.expected_identifier()?;
        let predicate = self.parse_predicate()?;

        Ok(Statement::Delete {
            table_name,
            predicate,
        })
    }
}
