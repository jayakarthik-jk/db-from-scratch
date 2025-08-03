use super::Statement;
use crate::{
    frontend::{
        lexer::{keyword::Keyword, token::TokenKind, LexerError, Token},
        parser::error::{ParserError, ParserErrorKind},
    },
    match_token, unwrap_ident, unwrap_ok,
    util::layer::Layer,
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_drop_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Table));
        let table_name = unwrap_ok!(unwrap_ident!(self.get_next_token()));
        Some(Ok(Statement::Drop { table_name }))
    }
}
