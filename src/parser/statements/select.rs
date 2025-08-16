use super::Statement;
use crate::{
    common::layer::Layer,
    error::DBError,
    lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, Token},
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, DBError>,
{
    pub(crate) fn parse_select_statement(&mut self) -> Result<Statement, DBError> {
        let expressions = self.parse_separated_expressions(Symbol::Comma)?;

        let from_token = self.get_next_token()?;
        if from_token.kind != TokenKind::Keyword(Keyword::From) {
            self.tokens.rewind(from_token);
            return Ok(Statement::Select {
                select_expressions: expressions,
                from: None,
                predicate: None,
            });
        }

        let table_name = self.expected_identifier()?;

        let predicate = self.parse_predicate()?;

        Ok(Statement::Select {
            select_expressions: expressions,
            from: Some(table_name),
            predicate,
        })
    }
}
