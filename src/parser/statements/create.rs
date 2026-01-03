use super::{Column, Statement};
use crate::{
    error::DBError,
    lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, Token},
    parser::datatype::Datatype,
    Parser,
};

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn parse_create_statement(&mut self) -> Result<Statement, DBError> {
        self.expect(TokenKind::Keyword(Keyword::Create))?;

        let table_name = self.expected_identifier()?;

        self.expect(TokenKind::Symbol(Symbol::OpenParanthesis))?;

        let columns = self.parse_seperated(Symbol::Comma, |parser| {
            parser.parse_create_statement_column()
        })?;

        self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;

        Ok(Statement::CreateTable {
            table_name,
            columns,
        })
    }

    pub(crate) fn parse_create_statement_column(&mut self) -> Result<Column, DBError> {
        let ident = self.expected_identifier()?;

        let data_type_token = self.get_next_token()?;
        let data_type = match data_type_token.kind {
            TokenKind::Keyword(keyword) => Datatype::from_keyword(keyword),
            _ => {
                return Err(DBError::KeywordExpected(data_type_token));
            }
        };

        let Some(data_type) = data_type else {
            return Err(DBError::DatatypeExpected(data_type_token));
        };
        Ok(Column {
            name: ident,
            data_type,
        })
    }
}
