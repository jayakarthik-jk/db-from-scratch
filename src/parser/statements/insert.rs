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
    pub(crate) fn parse_insert_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Into))?;

        let table_name = self.expected_identifier()?;

        let mut columns = None;

        if self.next_if(TokenKind::Symbol(Symbol::OpenParanthesis)).is_some() {
            let column_names =
                self.parse_seperated(Symbol::Comma, |parser| parser.expect_ident())?;
            self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
            columns = Some(column_names);
        }

        self.expect(TokenKind::Keyword(Keyword::Values))?;
        self.expect(TokenKind::Symbol(Symbol::OpenParanthesis))?;

        let values = self.parse_seperated(Symbol::Comma, |parser| parser.parse_expression())?;

        self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;

        Ok(Statement::Insert {
            table_name,
            columns,
            values,
        })
    }
}
