use super::{Column, Statement};
use crate::{
    frontend::{
        lexer::{
            keyword::Keyword,
            symbol::Symbol,
            token::{Identifier, TokenKind},
            LexerError, Token,
        },
        parser::error::{IntoParseResult, ParserError, ParserErrorKind},
    },
    match_token, unwrap_ident, unwrap_ok,
    util::layer::Layer,
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AlterType {
    Add(Column),
    Drop(Identifier),
    Modify(Column),
    Rename { old: Identifier, new: Identifier },
}

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_alter_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Table));

        let table_name = unwrap_ok!(unwrap_ident!(self.get_next_token()));

        let alter_types =
            unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser.parse_alter_type()));

        return Some(Ok(Statement::Alter {
            table_name,
            alter_types,
        }));
    }

    pub(crate) fn parse_alter_type(&mut self) -> Option<Result<AlterType, ParserError>> {
        let Token {
            kind: TokenKind::Keyword(keyword),
            ..
        } = unwrap_ok!(self.get_next_token())
        else {
            return "Alter Type ADD, DROP, MODIFY, or RENAME expected".as_err();
        };

        let alter_type = match keyword {
            Keyword::Add => {
                match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Column));
                let column = unwrap_ok!(self.parse_create_statement_column());
                AlterType::Add(column)
            }
            Keyword::Drop => {
                match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Column));
                let column_name_token = unwrap_ok!(self.get_next_token());
                if let TokenKind::Identifier(column_name) = column_name_token.kind {
                    AlterType::Drop(column_name)
                } else {
                    return "Table name expected after DROP COLUMN".as_err();
                }
            }
            Keyword::Modify => {
                match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Column));
                let column = unwrap_ok!(self.parse_create_statement_column());
                AlterType::Modify(column)
            }
            Keyword::Rename => {
                match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Column));
                let old_name_token = unwrap_ok!(self.get_next_token());
                let Token {
                    kind: TokenKind::Identifier(old),
                    ..
                } = old_name_token
                else {
                    return "Old column name expected after RENAME COLUMN".as_err();
                };
                match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::To));

                let new_name_token = unwrap_ok!(self.get_next_token());
                let Token {
                    kind: TokenKind::Identifier(new),
                    ..
                } = new_name_token
                else {
                    return "New column name expected after RENAME COLUMN".as_err();
                };
                AlterType::Rename { old, new }
            }

            keyword => {
                return Some(Err(ParserErrorKind::UnExpectedAlterType {
                    expected: Keyword::Add,
                    found: TokenKind::Keyword(keyword),
                }
                .into()))
            }
        };

        Some(Ok(alter_type))
    }
}
