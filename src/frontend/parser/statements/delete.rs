use super::Statement;
use crate::{
    frontend::{
        lexer::{keyword::Keyword, token::TokenKind, LexerError, Token},
        parser::error::ParserError,
    },
    match_token, unwrap_ok,
    util::layer::Layer,
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_delete_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Keyword(Keyword::From)
        ));

        let table_name = unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Identifier(ident),
            ident
        ));

        let mut predicate = None;
        match self.get_next_token()? {
            Err(e) => return Some(Err(e)),
            Ok(Token {
                kind: TokenKind::Keyword(Keyword::Where),
                ..
            }) => {
                predicate = Some(unwrap_ok!(self.parse_expression()));
            }
            Ok(token) => {
                self.tokens.rewind(token);
            }
        }

        return Some(Ok(Statement::Delete {
            table_name,
            predicate,
        }));
    }
}
