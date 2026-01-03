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
    pub(crate) fn parse_drop_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Table))?;
        let table_name = self.expected_identifier()?;
        Ok(Statement::DropTable { table_name })
    }
}
