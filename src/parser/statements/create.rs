use super::{Column, Statement};
use crate::{
    common::layer::Layer,
    Parser,
    {
        lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token},
        parser::{datatype::Datatype, error::ParserError},
    },
};

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn parse_create_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenKind::Keyword(Keyword::Create))?;

        let table_name = self.expected_identifier()?;

        self.expect(TokenKind::Symbol(Symbol::OpenParanthesis))?;

        let columns = self.parse_seperated(Symbol::Comma, |parser| {
            parser.parse_create_statement_column()
        })?;

        self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;

        Ok(Statement::Create {
            table_name,
            columns,
        })
    }

    pub(crate) fn parse_create_statement_column(&mut self) -> Result<Column, ParserError> {
        let ident = self.expected_identifier()?;

        let data_type_token = self.get_next_token()?;
        let data_type = match data_type_token.kind {
            TokenKind::Keyword(keyword) => Datatype::from_keyword(keyword),
            _ => {
                return Err(ParserError::KeywordExpected(data_type_token));
            }
        };

        let Some(data_type) = data_type else {
            return Err(ParserError::DatatypeExpected(data_type_token));
        };
        Ok(Column {
            name: ident,
            data_type,
        })
    }
}
