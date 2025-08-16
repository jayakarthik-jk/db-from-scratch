pub(crate) mod datatype;
pub(crate) mod error;
pub(crate) mod expression;
pub(crate) mod operators;
pub(crate) mod statements;

use super::lexer::{
    keyword::Keyword,
    symbol::Symbol,
    token::{Identifier, TokenKind},
    LexerError, Token,
};
use crate::{match_token, unwrap_ok, util::layer::Layer};
use error::{ParserError, ParserErrorKind};
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
        let keyword = unwrap_ok!(match_token!(
            self.get_next_token(),
            TokenKind::Keyword(keyword),
            keyword
        ));

        let statement = unwrap_ok!(match keyword {
            Keyword::Create => self.parse_create_statement(),
            Keyword::Alter => self.parse_alter_statement(),
            Keyword::Drop => self.parse_drop_statement(),
            Keyword::Insert => self.parse_insert_statement(),
            Keyword::Select => self.parse_select_statement(),
            Keyword::Update => self.parse_update_statement(),
            Keyword::Delete => self.parse_delete_statement(),
            _ => return Some(Err(ParserErrorKind::UnexpectedStatement.into())),
        });

        match_token!(self.get_next_token(), TokenKind::Symbol(Symbol::Semicolon));
        Some(Ok(statement))
    }
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

    fn parse_expression(&mut self) -> Option<Result<Expression, ParserError>> {
        self.parse_expression_of(BinaryOperator::max_precedence())
    }

    fn parse_expression_of(&mut self, precedence: u8) -> Option<Result<Expression, ParserError>> {
        if precedence == 0 {
            return self.parse_factor();
        }

        // Handle NOT operator
        let token = unwrap_ok!(self.get_next_token());
        if let TokenKind::Keyword(Keyword::Not) = token.kind {
            let next_expression = unwrap_ok!(self.parse_expression_of(precedence - 1));
            return Some(Ok(Expression::Negation(Box::new(next_expression))));
        } else {
            self.tokens.rewind(token);
        }

        let mut left = unwrap_ok!(self.parse_expression_of(precedence - 1));

        loop {
            let binary_operator = match BinaryOperator::parse_binary_operator(self, precedence) {
                Some(Ok(operator)) => operator,
                Some(Err(err)) => return Some(Err(err)),
                None => break,
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

    fn parse_identifier(&mut self) -> Option<Result<Identifier, ParserError>> {
        match unwrap_ok!(self.get_next_token()) {
            Token {
                kind: TokenKind::Identifier(ident),
                ..
            } => return Some(Ok(ident)),
            token => Some(Err(ParserErrorKind::Unexpected(token).into())),
        }
    }

    fn parse_factor(&mut self) -> Option<Result<Expression, ParserError>> {
        let token = unwrap_ok!(self.get_next_token());
        match token.kind {
            TokenKind::Literal(literal) => Some(Ok(Expression::Literal(literal))),
            TokenKind::Identifier(ident) => {
                let next_token = unwrap_ok!(self.get_next_token());
                if let TokenKind::Symbol(Symbol::OpenParanthesis) = next_token.kind {
                    // Function call
                    let expressions = unwrap_ok!(self.parse_separated_expressions(Symbol::Comma));
                    let close_paren_token = unwrap_ok!(self.get_next_token());
                    if close_paren_token.kind != TokenKind::Symbol(Symbol::CloseParanthesis) {
                        return Some(Err(ParserErrorKind::Unexpected(close_paren_token).into()));
                    }
                    return Some(Ok(Expression::FunctionCall {
                        ident,
                        arguments: expressions,
                    }));
                } else {
                    // Just an identifier
                    self.tokens.rewind(next_token);
                    return Some(Ok(Expression::Identifier(ident)));
                }
            }
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
            TokenKind::Symbol(Symbol::Star) => {
                // This is a special case for `SELECT *`
                // We treat it as a wildcard expression
                Some(Ok(Expression::Wildcard))
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
        self.parse_seperated(separator, |parser| parser.parse_expression())
    }

    fn parse_seperated<Callback, ReturnType>(
        &mut self,
        separator: Symbol,
        callback: Callback,
    ) -> Option<Result<Vec<ReturnType>, ParserError>>
    where
        Callback: Fn(&mut Self) -> Option<Result<ReturnType, ParserError>>,
    {
        let mut expressions = Vec::new();

        loop {
            let expr = unwrap_ok!(callback(self));
            expressions.push(expr);
            // Ensure separator before each expression
            let token = unwrap_ok!(self.get_next_token());

            if let TokenKind::Symbol(symbol) = token.kind {
                if symbol == separator {
                    continue;
                }
            }
            self.tokens.rewind(token);
            break;
        }

        Some(Ok(expressions))
    }
}
