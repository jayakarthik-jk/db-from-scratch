use crate::{match_token, unwrap_ok, util::layer::Layer};

use super::lexer::{keyword::Keyword, symbol::Symbol, token::TokenKind, LexerError, Token};
use alter_type::AlterType;
use datatype::Datatype;
use expression::{AssignmentOperator, BinaryOperator, Expression};
use parser_error::{IntoParseResult, ParserError, ParserErrorKind};
use statement::{Column, Statement};

pub(crate) mod alter_type;
pub(crate) mod datatype;
pub(crate) mod expression;
pub(crate) mod parser_error;
pub(crate) mod statement;

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
        let token = unwrap_ok!(self.get_next_token());
        let TokenKind::Keyword(keyword) = token.kind else {
            return Some(Err(ParserErrorKind::KeywordExpected(token).into()));
        };
        let statement = unwrap_ok!(match keyword {
            Keyword::Select => self.parse_select_statement(),
            Keyword::Create => self.parse_create_statement(),
            Keyword::Alter => self.parse_alter_statement(),
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

    fn parse_select_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        let expressions = unwrap_ok!(self.parse_separated_expressions(Symbol::Comma));

        let from_token = unwrap_ok!(self.get_next_token());
        if from_token.kind != TokenKind::Keyword(Keyword::From) {
            self.tokens.rewind(from_token);
            return Some(Ok(Statement::Select {
                select_expressions: expressions,
                from: None,
            }));
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
        return Some(Ok(Statement::Select {
            select_expressions: expressions,
            from: Some(table_name),
        }));
    }

    pub(crate) fn parse_create_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Table));

        let table_name_token = unwrap_ok!(self.get_next_token());
        let table_name = match table_name_token.kind {
            TokenKind::Identifier(ident) => ident,
            _ => {
                return Some(Err(
                    ParserErrorKind::TableNameExpected(table_name_token).into()
                ))
            }
        };

        match_token!(
            self.get_next_token(),
            TokenKind::Symbol(Symbol::OpenParanthesis)
        );

        let columns = unwrap_ok!(self.parse_seperated(Symbol::Comma, |parser| parser
            .parse_create_statement_column()));

        match_token!(
            self.get_next_token(),
            TokenKind::Symbol(Symbol::CloseParanthesis)
        );

        Some(Ok(Statement::Create {
            table_name,
            columns,
        }))
    }

    pub(crate) fn parse_create_statement_column(&mut self) -> Option<Result<Column, ParserError>> {
        let ident_token = unwrap_ok!(self.get_next_token());
        let ident = match ident_token.kind {
            TokenKind::Identifier(ident) => ident,
            _ => {
                return Some(Err(ParserErrorKind::Unexpected(ident_token).into()));
            }
        };

        let data_type_token = unwrap_ok!(self.get_next_token());
        let data_type = match data_type_token.kind {
            TokenKind::Keyword(keyword) => Datatype::from_keyword(keyword),
            _ => {
                return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
            }
        };
        if data_type.is_none() {
            return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
        }
        if let Some(data_type) = data_type {
            return Some(Ok(Column {
                name: ident,
                data_type,
            }));
        } else {
            return Some(Err(ParserErrorKind::Unexpected(data_type_token).into()));
        }
    }

    pub(crate) fn parse_alter_statement(&mut self) -> Option<Result<Statement, ParserError>> {
        match_token!(self.get_next_token(), TokenKind::Keyword(Keyword::Table));
        let table_name_token = unwrap_ok!(self.get_next_token());
        let table_name = match table_name_token.kind {
            TokenKind::Identifier(ident) => ident,
            _ => {
                return Some(Err(
                    ParserErrorKind::TableNameExpected(table_name_token).into()
                ))
            }
        };

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
