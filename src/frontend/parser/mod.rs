use crate::{unwrap_ok, util::layer::Layer};

use super::lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token};
use expression::{AssignmentOperator, BinaryOperator, Expression};
use parser_error::{ParserError, ParserErrorKind};
use statement::{SelectStatement, Statement};

pub(crate) mod expression;
pub(crate) mod parser_error;
pub(crate) mod statement;

pub(crate) struct Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    tokens: TokenLayer,
}

impl<TokenLayer> Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    pub(crate) fn new(tokens: TokenLayer) -> Self {
        Self { tokens }
    }

    fn get_next_token(&mut self) -> Option<Result<Token, ParserError>> {
        Some(
            self.tokens
                .next()?
                .map_err(|err| ParserErrorKind::LexerError(err).into()),
        )
    }

    /// <ident> <assignment_operator> <expression>
    fn parse_assignment_expression(&mut self) -> Option<Result<Expression, ParserError>> {
        // TODO
        let first = unwrap_ok!(self.get_next_token());

        let ident = match &first.kind {
            TokenKind::Identifier(ident) => ident.to_owned(),
            _ => {
                self.tokens.rewind(first);
                return self.parse_expression();
            }
        };

        let second = unwrap_ok!(self.get_next_token());

        let symbol = match second.kind {
            TokenKind::Symbol(symbol) => symbol,
            _ => {
                self.tokens.rewind(second);
                self.tokens.rewind(first);
                return self.parse_expression();
            }
        };

        let Some(assignment_operator) = AssignmentOperator::match_symbol(symbol) else {
            self.tokens.rewind(second);
            self.tokens.rewind(first);
            return self.parse_expression();
        };

        let expression = unwrap_ok!(self.parse_expression());

        Some(Ok(Expression::Assign {
            ident,
            operator: assignment_operator,
            right: Box::new(expression),
        }))
    }

    fn parse_expression(&mut self) -> Option<Result<Expression, ParserError>> {
        self.parse_expression_of(BinaryOperator::max_precedence())
    }

    fn parse_expression_of(&mut self, precedence: u8) -> Option<Result<Expression, ParserError>> {
        if precedence == 0 {
            return self.parse_factor();
        }
        let mut left = unwrap_ok!(self.parse_expression_of(precedence - 1));

        loop {
            let token = unwrap_ok!(self.get_next_token());
            let symbol = match token.kind {
                TokenKind::Symbol(symbol) => symbol,
                _ => {
                    self.tokens.rewind(token);
                    break;
                }
            };

            let binary_operator =
                match BinaryOperator::match_symbol_with_precedence(symbol, precedence) {
                    Some(operator) => operator,
                    _ => {
                        self.tokens.rewind(token);
                        break;
                    }
                };

            let right = unwrap_ok!(self.parse_expression_of(precedence - 1));

            left = Expression::Binary {
                left: Box::new(left),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        Some(Ok(left))
    }

    fn parse_factor(&mut self) -> Option<Result<Expression, ParserError>> {
        let token = unwrap_ok!(self.get_next_token());
        match token.kind {
            TokenKind::Literal(literal) => Some(Ok(Expression::Literal(literal))),
            TokenKind::Identifier(ident) => Some(Ok(Expression::Identifier(ident))),
            TokenKind::Symbol(Symbol::OpenParanthesis) => {
                let expression = unwrap_ok!(self.parse_expression());

                Some(match self.get_next_token()? {
                    Ok(Token {
                        kind: TokenKind::Symbol(Symbol::CloseParanthesis),
                        ..
                    }) => Ok(expression),
                    Ok(token) => Err(ParserErrorKind::Unexpected(token).into()),
                    Err(err) => Err(err),
                })
            }
            _ => {
                self.tokens.rewind(token);
                Some(Err(ParserErrorKind::NotAnExpression.into()))
            }
        }
    }

    fn parse_separated_expressions(
        &mut self,
        separator: Symbol,
    ) -> Option<Result<Vec<Expression>, ParserError>> {
        let mut expressions = Vec::new();
        let mut first = true;

        loop {
            let expr = match self.parse_assignment_expression()? {
                Ok(e) => e,
                Err(err) if matches!(err.kind, ParserErrorKind::NotAnExpression) => break,
                Err(err) => return Some(Err(err)),
            };

            if !first {
                // Ensure separator before each expression
                match self.get_next_token()? {
                    Ok(Token {
                        kind: TokenKind::Symbol(sym),
                        ..
                    }) if sym == separator => (),
                    Ok(tok) => {
                        self.tokens.rewind(tok);
                        return Some(Err(ParserErrorKind::MissingSeperator {
                            seperator: separator,
                        }
                        .into()));
                    }
                    Err(err) => return Some(Err(err)),
                };
            }

            expressions.push(expr);
            first = false;
        }

        Some(Ok(expressions))
    }

    fn parse_select_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        let expressions = unwrap_ok!(self.parse_separated_expressions(Symbol::Comma));
        // parse the `FROM` clause
        let from_token = unwrap_ok!(self.get_next_token());
        if from_token.kind != TokenKind::Keyword(Keyword::From) {
            return Some(Err(ParserErrorKind::KeywordExpected(from_token).into()));
        }

        let table_name_token = unwrap_ok!(self.get_next_token());

        let table_name = match table_name_token.kind {
            TokenKind::Identifier(ident) => ident,
            _ => {
                return Some(Err(
                    ParserErrorKind::TableNameExpected(table_name_token).into()
                ))
            }
        };
        return Some(Ok(Statement::Select(SelectStatement {
            select_expressions: expressions,
            from: Some(table_name),
        })));
    }

    fn parse_insert_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
}

impl<TokenLayer> Iterator for Parser<TokenLayer>
where
    TokenLayer: Layer<Token, LexerError>,
{
    type Item = Result<Statement, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = unwrap_ok!(self.get_next_token());
        let TokenKind::Keyword(keyword) = token.kind else {
            return Some(Err(ParserErrorKind::KeywordExpected(token).into()));
        };

        Some(match keyword {
            Keyword::Select => self.parse_select_statement()?,
            Keyword::Insert => self.parse_insert_statement(),
            _ => return Some(Err(ParserErrorKind::UnexpectedStatement.into())),
        })
    }
}
