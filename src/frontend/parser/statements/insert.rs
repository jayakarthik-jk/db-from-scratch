use super::Statement;
use crate::{
    frontend::{
        lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token},
        parser::error::{ParserError, ParserErrorKind},
    },
    match_token, unwrap_ok,
    util::layer::Layer,
    Parser,
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_insert_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Into));

        let table_name = unwrap_ok!(match_token!(self.get_next_token(), TokenKind::Identifier(ident), ident));

        let mut columns = None;

        let token = unwrap_ok!(self.get_next_token());

        if let TokenKind::Symbol(Symbol::OpenParanthesis) = token.kind {
            let column_names =
                unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser.parse_identifier()));
            match_token!(
                self.get_next_token(),
                TokenKind::Symbol(Symbol::CloseParanthesis)
            );
            columns = Some(column_names);
        } else {
            self.tokens.rewind(token);
        }

        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Values));
        match_token!(self.get_next_token(), TokenKind::Symbol(Symbol::OpenParanthesis));

        let values =
            unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser.parse_expression()));

        match_token!(self.get_next_token(), TokenKind::Symbol(Symbol::CloseParanthesis));
        Some(Ok(Statement::Insert {
            table_name,
            columns,
            values,
        }))
    }
}
