use super::{Column, Statement};
use crate::{
    frontend::{
        lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token},
        parser::{
            datatype::Datatype,
            error::{ParserError, ParserErrorKind},
        },
    },
    match_token, unwrap_ok,
    util::layer::Layer,
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_create_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Table));

        let table_name = unwrap_ok!(match_token!(self.get_next_token(), TokenKind::Identifier(ident), ident));

        match_token!(
            self.get_next_token(),
            TokenKind::Symbol(Symbol::OpenParanthesis)
        );

        let columns = unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser
            .parse_create_statement_column()));

        match_token!(
            self.get_next_token(),
            TokenKind::Symbol(Symbol::CloseParanthesis)
        );

        Some(Ok(Statement::Create {
            table_name,
            columns,
        }))
    }

    pub(crate) fn parse_create_statement_column(&mut self) -> Option<Result<Column, ParserError>> {
        let ident = unwrap_ok!(match_token!(self.get_next_token(), TokenKind::Identifier(ident), ident));

        let data_type_token = unwrap_ok!(self.get_next_token());
        let data_type = match data_type_token.kind {
            TokenKind::Keyword(keyword) => Datatype::from_keyword(keyword),
            _ => {
                return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
            }
        };
        if data_type.is_none() {
            return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
        }
        if let Some(data_type) = data_type {
            return Some(Ok(Column {
                name: ident,
                data_type,
            }));
        } else {
            return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
        }
    }
}
