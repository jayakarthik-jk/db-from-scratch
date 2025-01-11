use std::{collections::VecDeque, io::Read};

use super::lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, Lexer, Token};
use expression::{AssignmentOperator, BinaryOperator, Expression};
use parser_error::{AddError, ParserError, ParserErrorKind};
use statement::{SelectStatement, Statement};

pub(crate) mod expression;
pub(crate) mod parser_error;
pub(crate) mod statement;

pub(crate) struct Parser<T>
where
    T: Read,
{
    lexer: Lexer<T>,
    buffer: VecDeque<Token>,
}

impl<T> Parser<T>
where
    T: Read,
{
    pub(crate) fn new(input: T) -> Self {
        Self {
            lexer: Lexer::new(input),
            buffer: VecDeque::with_capacity(3),
        }
    }

    fn get_next_token(&mut self) -> Result<Token, ParserError> {
        if let Some(token) = self.buffer.pop_back() {
            return Ok(token);
        }
        self.lexer.next().map_err(|err| {
            match err {
                super::lexer::LexerError::EOF => ParserErrorKind::EOF,
                err => ParserErrorKind::LexerError(err),
            }
            .into()
        })
    }

    fn rewind(&mut self, token: Token) {
        self.buffer.push_back(token)
    }

    /// <ident> <assignment_operator> <expression>
    fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        let first = self.get_next_token()?;

        let ident = match &first.kind {
            TokenKind::Identifier(ident) => ident.to_owned(),
            _ => {
                self.rewind(first);
                return self.parse_expression();
            }
        };

        let second = self.get_next_token()?;

        let symbol = match second.kind {
            TokenKind::Symbol(symbol) => symbol,
            _ => {
                self.rewind(second);
                self.rewind(first);
                return self.parse_expression();
            }
        };

        let Some(assignment_operator) = AssignmentOperator::match_symbol(symbol) else {
            self.rewind(second);
            self.rewind(first);
            return self.parse_expression();
        };

        let expression = match self.parse_expression() {
            Ok(expression) => expression,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(Expression::Assign {
            ident,
            operator: assignment_operator,
            right: Box::new(expression),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression_of(BinaryOperator::max_precedence())
    }

    fn parse_expression_of(&mut self, precedence: u8) -> Result<Expression, ParserError> {
        if precedence == 0 {
            return self.parse_factor();
        }
        let mut left = self.parse_expression_of(precedence - 1)?;

        loop {
            let token = self.get_next_token()?;
            let symbol = match token.kind {
                TokenKind::Symbol(symbol) => symbol,
                _ => {
                    self.rewind(token);
                    break;
                }
            };

            let binary_operator =
                match BinaryOperator::match_symbol_with_precedence(symbol, precedence) {
                    Some(operator) => operator,
                    _ => {
                        self.rewind(token);
                        break;
                    }
                };

            let right = match self.parse_expression_of(precedence - 1) {
                Ok(token) => token,
                Err(ParserError {
                    kind: ParserErrorKind::NotAnExpression,
                    ..
                }) => {
                    return Err(ParserErrorKind::IncompleteExpression(token).into());
                }
                Err(err) => return Err(err),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let token = self.get_next_token()?;
        match token.kind {
            TokenKind::Literal(literal) => Ok(Expression::Literal(literal)),
            TokenKind::Identifier(ident) => Ok(Expression::Identifier(ident)),
            TokenKind::Symbol(Symbol::OpenParanthesis) => {
                let expression = self.parse_expression()?;

                match self.get_next_token() {
                    Ok(Token {
                        kind: TokenKind::Symbol(Symbol::CloseParanthesis),
                        ..
                    }) => Ok(expression),
                    Ok(token) => Err(ParserErrorKind::Unexpected(token).into()),
                    Err(err) => Err(err),
                }
            }
            _ => {
                self.rewind(token);
                Err(ParserErrorKind::NotAnExpression.into())
            }
        }
    }

    fn parse_seperated_expression(
        &mut self,
        seperator: Symbol,
    ) -> (Vec<Expression>, Option<ParserError>) {
        let mut expressions = Vec::new();
        let mut error = None;

        let mut is_first_iter = true;
        let mut seperator_exists = false;
        loop {
            match self.parse_assignment_expression() {
                Ok(expression) => {
                    if !is_first_iter && !seperator_exists {
                        error.add(ParserErrorKind::MissingSeperator {
                            //position,
                            seperator,
                        });
                    }
                    is_first_iter = false;
                    expressions.push(expression);
                    match self.get_next_token() {
                        Ok(Token {
                            kind: TokenKind::Symbol(symbol),
                            ..
                        }) if symbol == seperator => {
                            seperator_exists = true;
                            continue;
                        }
                        Ok(token) => {
                            self.rewind(token);
                        }
                        Err(err) => {
                            error.add_all(err);
                        }
                    }
                    seperator_exists = false;
                }
                Err(ParserError {
                    kind: ParserErrorKind::NotAnExpression,
                    ..
                }) => break,
                Err(err) => {
                    error.add_all(err);
                    break;
                }
            }
        }

        (expressions, error)
    }

    fn parse_select_statement(&mut self) -> Result<Statement, ParserError> {
        let (expressions, errors) = self.parse_seperated_expression(Symbol::Comma);
        if let Some(err) = errors {
            return Err(err);
        }
        _ = self.get_next_token();
        Ok(Statement::Select(SelectStatement {
            select_expressions: expressions,
            from: None,
        }))
    }

    fn parse_insert_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
}

impl<T> Iterator for Parser<T>
where
    T: Read,
{
    type Item = Result<Statement, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        let keyword = match self.get_next_token() {
            Ok(Token {
                kind: TokenKind::Keyword(keyword),
                ..
            }) => keyword,
            Ok(token) => return Some(Err(ParserErrorKind::KeywordExpected(token).into())),
            Err(ParserError {
                kind: ParserErrorKind::EOF,
                ..
            }) => return None,
            Err(err) => return Some(Err(err)),
        };

        let result = match keyword {
            Keyword::Select => self.parse_select_statement(),
            Keyword::Insert => self.parse_insert_statement(),
            _ => return Some(Err(ParserErrorKind::UnexpectedStatement.into())),
        };

        if let Err(ParserError {
            kind: ParserErrorKind::EOF,
            ..
        }) = result
        {
            None
        } else {
            Some(result)
        }
    }
}
