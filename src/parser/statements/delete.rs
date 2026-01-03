use super::Statement;
use crate::{
    error::DBError,
    lexer::{keyword::Keyword, token::TokenKind, Token},
    Parser,
};

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_delete_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::From))?;
        let table_name = self.expect_identifier()?;
        let predicate = self.parse_predicate()?;

        Ok(Statement::Delete {
            table_name,
            predicate,
        })
    }
}
