use crate::{
    frontend::{
        lexer::{
            keyword::Keyword,
            symbol::Symbol,
            token::{Identifier, TokenKind},
            LexerError, Token,
        },
        parser::{error::ParserError, expression::Expression},
    },
    match_token, unwrap_ok,
    util::layer::Layer,
    Parser,
};

use super::Statement;

#[derive(Debug, PartialEq)]
pub(crate) struct UpdateSet {
    pub(crate) column: Identifier,
    pub(crate) value: Expression,
}

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_update_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        let table_name = unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Identifier(ident),
            ident
        ));

        unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Keyword(Keyword::Set)
        ));

        let update_set =
            unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser.parse_update_set()));

        let predicate = match self.get_next_token()? {
            Err(err) => return Some(Err(err)),
            Ok(Token {
                kind: TokenKind::Keyword(Keyword::Where),
                ..
            }) => Some(unwrap_ok!(self.parse_expression())),
            Ok(token) => {
                self.tokens.rewind(token);
                None
            }
        };

        Some(Ok(Statement::Update {
            table_name,
            set: update_set,
            predicate,
        }))
    }

    pub(crate) fn parse_update_set(&mut self) -> Option<Result<UpdateSet, ParserError>> {
        let column = unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Identifier(ident),
            ident
        ));

        unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Symbol(Symbol::Equal)
        ));

        let value = unwrap_ok!(self.parse_expression());

        Some(Ok(UpdateSet { column, value }))
    }
}
