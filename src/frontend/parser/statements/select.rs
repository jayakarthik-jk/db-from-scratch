use super::Statement;
use crate::{
    frontend::{
        lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token},
        parser::error::ParserError,
    }, match_token, unwrap_ok, util::layer::Layer, Parser
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_select_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        let expressions = unwrap_ok!(self.parse_separated_expressions(Symbol::Comma));

        let from_token = unwrap_ok!(self.get_next_token());
        if from_token.kind != TokenKind::Keyword(Keyword::From) {
            self.tokens.rewind(from_token);
            return Some(Ok(Statement::Select {
                select_expressions: expressions,
                from: None,
            }));
        }

        let table_name = unwrap_ok!(match_token!(self.get_next_token(), TokenKind::Identifier(ident), ident));

        return Some(Ok(Statement::Select {
            select_expressions: expressions,
            from: Some(table_name),
        }));
    }
}
