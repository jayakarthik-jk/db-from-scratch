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
    pub(crate) fn parse_insert_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Into))?;

        let table_name = self.expected_identifier()?;

        let mut columns = None;

        let token = self.get_next_token()?;

        if let TokenKind::Symbol(Symbol::OpenParanthesis) = token.kind {
            let column_names =
                self.parse_seperated(Symbol::Comma, |parser| parser.expect_ident())?;
            self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
            columns = Some(column_names);
        } else {
            self.tokens.rewind(token);
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
