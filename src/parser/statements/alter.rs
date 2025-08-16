use super::{Column, Statement};
use crate::{
    error::{DBError, IntoParseResult},
    lexer::{
        keyword::Keyword,
        symbol::Symbol,
        token::{Ident, TokenKind},
        Token,
    },
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AlterType {
    Add(Column),
    Drop(Ident),
    Modify(Column),
    Rename { old: Ident, new: Ident },
}

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_alter_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Table))?;

        let table_name = self.expected_identifier()?;

        let alter_types =
            self.parse_seperated(Symbol::Comma, |parser| parser.parse_alter_type())?;

        Ok(Statement::Alter {
            table_name,
            alter_types,
        })
    }

    pub(crate) fn parse_alter_type(&mut self) -> Result<AlterType, DBError> {
        let Token {
            kind: TokenKind::Keyword(keyword),
            ..
        } = self.get_next_token()?
        else {
            return "Alter Type ADD, DROP, MODIFY, or RENAME expected".as_err();
        };

        let alter_type = match keyword {
            Keyword::Add => {
                self.expect(TokenKind::Keyword(Keyword::Column))?;
                let column = self.parse_create_statement_column()?;
                AlterType::Add(column)
            }
            Keyword::Drop => {
                self.expect(TokenKind::Keyword(Keyword::Column))?;
                let column_name_token = self.get_next_token()?;
                if let TokenKind::Ident(column_name) = column_name_token.kind {
                    AlterType::Drop(column_name)
                } else {
                    return "Table name expected after DROP COLUMN".as_err();
                }
            }
            Keyword::Modify => {
                self.expect(TokenKind::Keyword(Keyword::Column))?;
                let column = self.parse_create_statement_column()?;
                AlterType::Modify(column)
            }
            Keyword::Rename => {
                self.expect(TokenKind::Keyword(Keyword::Column))?;
                let old_name_token = self.get_next_token()?;
                let Token {
                    kind: TokenKind::Ident(old),
                    ..
                } = old_name_token
                else {
                    return "Old column name expected after RENAME COLUMN".as_err();
                };
                self.expect(TokenKind::Keyword(Keyword::To))?;

                let new_name_token = self.get_next_token()?;
                let Token {
                    kind: TokenKind::Ident(new),
                    ..
                } = new_name_token
                else {
                    return "New column name expected after RENAME COLUMN".as_err();
                };
                AlterType::Rename { old, new }
            }

            keyword => {
                return Err(DBError::UnExpectedAlterType {
                    expected: Keyword::Add,
                    found: TokenKind::Keyword(keyword),
                })
            }
        };

        Ok(alter_type)
    }
}
