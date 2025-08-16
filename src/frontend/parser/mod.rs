pub(crate) mod datatype;
pub(crate) mod error;
pub(crate) mod expression;
pub(crate) mod operators;
pub(crate) mod statements;

use super::lexer::{
    keyword::Keyword,
    symbol::Symbol,
    token::{Ident, TokenKind},
    LexerError, Token,
};
use crate::util::layer::Layer;
use error::ParserError;
use expression::Expression;
use operators::binary::BinaryOperator;
use statements::Statement;

pub(crate) struct Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    tokens: TokenLayer,
}

impl<TokenLayer> Iterator for Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    type Item = Result<Statement, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        let keyword = match self.expect_keyword_kind() {
            Ok(keyword) => keyword,
            Err(ParserError::Eof) => return None,
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
            _ => return Some(Err(ParserError::UnexpectedStatement)),
        };

        match &statement {
            Err(ParserError::Eof) => None,

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
                Err(ParserError::Eof) => Err(ParserError::UnexpectedToken {
                    expected: TokenKind::Symbol(Symbol::Semicolon),
                }),
                Err(err) => Err(err),
            }),
        }
    }
}

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn new(tokens: TokenLayer) -> Self {
        Self { tokens }
    }

    fn get_next_token(&mut self) -> Result<Token, ParserError> {
        self.tokens
            .next()
            .ok_or(ParserError::Eof)?
            .map_err(ParserError::LexerError)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {
        match self.get_next_token()? {
            Token { kind: actual, .. } if expected == actual => Ok(()),
            token => Err(ParserError::Unexpected {
                found: token,
                expected,
            }),
        }
    }

    fn expect_keyword_kind(&mut self) -> Result<Keyword, ParserError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Keyword(keyword),
                ..
            } => Ok(keyword),
            token => Err(ParserError::KeywordExpected(token)),
        }
    }

    fn expected_identifier(&mut self) -> Result<Ident, ParserError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Ident(ident),
                ..
            } => Ok(ident),
            token => Err(ParserError::IdentExpected(token)),
        }
    }

    fn parse_predicate(&mut self) -> Result<Option<Expression>, ParserError> {
        let mut predicate = None;
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Keyword(Keyword::Where),
                ..
            } => {
                predicate = Some(self.parse_expression()?);
            }
            token => {
                self.tokens.rewind(token);
            }
        }
        Ok(predicate)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression_of(BinaryOperator::max_precedence())
    }

    fn parse_expression_of(&mut self, precedence: u8) -> Result<Expression, ParserError> {
        if precedence == 0 {
            return self.parse_factor();
        }

        // Handle NOT operator
        let token = self.get_next_token()?;
        if let TokenKind::Keyword(Keyword::Not) = token.kind {
            let next_expression = self.parse_expression_of(precedence - 1)?;
            return Ok(Expression::Negation(Box::new(next_expression)));
        } else {
            self.tokens.rewind(token);
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

    fn expect_ident(&mut self) -> Result<Ident, ParserError> {
        match self.get_next_token()? {
            Token {
                kind: TokenKind::Ident(ident),
                ..
            } => Ok(ident),
            token => Err(ParserError::IdentExpected(token)),
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let token = self.get_next_token()?;
        match token.kind {
            TokenKind::Literal(literal) => Ok(Expression::Literal(literal)),
            TokenKind::Ident(ident) => {
                let next_token = self.get_next_token()?;
                if let TokenKind::Symbol(Symbol::OpenParanthesis) = next_token.kind {
                    // Function call
                    let expressions = self.parse_separated_expressions(Symbol::Comma)?;
                    self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
                    Ok(Expression::FunctionCall {
                        ident,
                        arguments: expressions,
                    })
                } else {
                    // Just an identifier
                    self.tokens.rewind(next_token);
                    Ok(Expression::Identifier(ident))
                }
            }
            TokenKind::Symbol(Symbol::OpenParanthesis) => {
                let expression = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::CloseParanthesis))?;
                Ok(expression)
            }
            TokenKind::Symbol(Symbol::Star) => {
                // This is a special case for `SELECT *`
                // We treat it as a wildcard expression
                Ok(Expression::Wildcard)
            }
            _ => {
                self.tokens.rewind(token);
                Err(ParserError::NotAnExpression)
            }
        }
    }

    fn parse_separated_expressions(
        &mut self,
        separator: Symbol,
    ) -> Result<Vec<Expression>, ParserError> {
        self.parse_seperated(separator, |parser| parser.parse_expression())
    }

    fn parse_seperated<Callback, ReturnType>(
        &mut self,
        separator: Symbol,
        callback: Callback,
    ) -> Result<Vec<ReturnType>, ParserError>
    where
        Callback: Fn(&mut Self) -> Result<ReturnType, ParserError>,
    {
        let mut expressions = Vec::new();

        loop {
            let expr = callback(self)?;
            expressions.push(expr);
            // Ensure separator before each expression
            let token = self.get_next_token()?;

            if let TokenKind::Symbol(symbol) = token.kind {
                if symbol == separator {
                    continue;
                }
            }
            self.tokens.rewind(token);
            break;
        }

        Ok(expressions)
    }
}
