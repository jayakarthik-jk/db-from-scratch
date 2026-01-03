pub(crate) mod datatype;
pub(crate) mod expression;
pub(crate) mod operators;
pub(crate) mod statements;

use std::iter::Peekable;

use super::lexer::{
    keyword::Keyword,
    symbol::Symbol,
    token::{Ident, TokenKind},
    Token,
};
use crate::{common::peekable_ext::ConsumeIf, error::DBError};
use expression::Expression;
use operators::binary::BinaryOperator;
use statements::Statement;

pub(crate) struct Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    tokens: Peekable<Tokens>,
}

impl<Tokens> Iterator for Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    type Item = Result<Statement, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let keyword = match self.expect_keyword_kind() {
            Ok(keyword) => keyword,
            Err(DBError::Eof) => return None,
            Err(err) => return Some(Err(err)),
        };

        let statement = match keyword {
            Keyword::Create => self.parse_create_statement(),
            Keyword::Alter => self.parse_alter_statement(),
            Keyword::Drop => self.parse_drop_statement(),
            Keyword::Insert => self.parse_insert_statement(),
            Keyword::Select => self.parse_select_statement(),
            Keyword::Update => self.parse_update_statement(),
            Keyword::Delete => self.parse_delete_statement(),
            _ => return Some(Err(DBError::UnexpectedStatement)),
        };

        match &statement {
            Err(DBError::Eof) => None,

            // traverse the tokens until the next Semicolon
            Err(_) => {
                while let Ok(token) = self.get_next_token() {
                    if token.kind == TokenKind::Symbol(Symbol::Semicolon) {
                        break;
                    }
                }
                Some(statement)
            }

            Ok(_) => Some(match self.expect(TokenKind::Symbol(Symbol::Semicolon)) {
                Ok(_) => statement,
                Err(DBError::Eof) => Err(DBError::UnexpectedEof {
                    expected: TokenKind::Symbol(Symbol::Semicolon),
                }),
                Err(err) => Err(err),
            }),
        }
    }
}

impl<Tokens> Parser<Tokens>
where
    Tokens: Iterator<Item = Result<Token, DBError>>,
{
    pub(crate) fn new(tokens: Tokens) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn get_next_token(&mut self) -> Result<Token, DBError> {
        self.tokens.next().ok_or(DBError::Eof)?
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), DBError> {
        match self.get_next_token()? {
            Token { kind: actual, .. } if expected == actual => Ok(()),
            token => Err(DBError::Unexpected {
                found: token,
                expected,
            }),
        }
    }

    fn consume_if(&mut self, expected: TokenKind) -> Option<Result<Token, DBError>> {
        self.tokens
            .consume_if(|token| matches!(token, Ok(Token { kind, .. }) if *kind == expected))
    }

    fn expect_keyword_kind(&mut self) -> Result<Keyword, DBError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Keyword(keyword),
                ..
            } => Ok(keyword),
            token => Err(DBError::KeywordExpected(token)),
        }
    }

    fn expected_identifier(&mut self) -> Result<Ident, DBError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Ident(ident),
                ..
            } => Ok(ident),
            token => Err(DBError::IdentExpected(token)),
        }
    }

    fn parse_predicate(&mut self) -> Result<Option<Expression>, DBError> {
        let predicate = if self
            .consume_if(TokenKind::Keyword(Keyword::Where))
            .is_some()
        {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(predicate)
    }

    fn parse_expression(&mut self) -> Result<Expression, DBError> {
        self.parse_expression_of(BinaryOperator::max_precedence())
    }

    fn parse_expression_of(&mut self, precedence: u8) -> Result<Expression, DBError> {
        if precedence == 0 {
            return self.parse_factor();
        }

        // Handle NOT operator
        if self.consume_if(TokenKind::Keyword(Keyword::Not)).is_some() {
            let next_expression = self.parse_expression_of(precedence - 1)?;
            return Ok(Expression::Negation(Box::new(next_expression)));
        }

        let mut left = self.parse_expression_of(precedence - 1)?;

        loop {
            let binary_operator = match BinaryOperator::parse_binary_operator(self, precedence) {
                Some(Ok(operator)) => operator,
                Some(Err(err)) => return Err(err),
                None => break,
            };

            let right = self.parse_expression_of(precedence - 1)?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn expect_ident(&mut self) -> Result<Ident, DBError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Ident(ident),
                ..
            } => Ok(ident),
            token => Err(DBError::IdentExpected(token)),
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, DBError> {
        let token = self.get_next_token()?;
        match token.kind {
            TokenKind::Literal(literal) => Ok(Expression::Literal(literal)),
            TokenKind::Ident(ident) => {
                if self
                    .consume_if(TokenKind::Symbol(Symbol::OpenParanthesis))
                    .is_some()
                {
                    let expressions = self.parse_separated_expressions(Symbol::Comma)?;
                    self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
                    Ok(Expression::FunctionCall {
                        ident,
                        arguments: expressions,
                    })
                } else {
                    Ok(Expression::Ident(ident))
                }
            }
            TokenKind::Symbol(Symbol::OpenParanthesis) => {
                let expression = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
                Ok(expression)
            }
            _ => Err(DBError::UnexpectedToken { found: token }),
        }
    }

    fn parse_separated_expressions(
        &mut self,
        separator: Symbol,
    ) -> Result<Vec<Expression>, DBError> {
        self.parse_seperated(separator, |parser| parser.parse_expression())
    }

    fn parse_seperated<Callback, ReturnType>(
        &mut self,
        separator: Symbol,
        callback: Callback,
    ) -> Result<Vec<ReturnType>, DBError>
    where
        Callback: Fn(&mut Self) -> Result<ReturnType, DBError>,
    {
        let mut expressions = Vec::new();

        loop {
            let expr = callback(self)?;
            expressions.push(expr);
            // Ensure separator before each expression
            if self.consume_if(TokenKind::Symbol(separator)).is_none() {
                break;
            }
        }

        Ok(expressions)
    }
}
