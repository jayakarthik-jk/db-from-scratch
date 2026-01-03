use super::Statement;
use crate::{
    error::DBError,
    lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, Token},
    Parser,
};

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_select_statement(&mut self) -> Result<Statement, DBError> {
        let expressions = self.parse_separated_expressions(Symbol::Comma)?;

        if self.consume_if(TokenKind::Keyword(Keyword::From)).is_none() {
            return Ok(Statement::Select {
                select_expressions: expressions,
                from: None,
                predicate: None,
            });
        }

        let table_name = self.expect_identifier()?;

        let predicate = self.parse_predicate()?;

        Ok(Statement::Select {
            select_expressions: expressions,
            from: Some(table_name),
            predicate,
        })
    }
}
