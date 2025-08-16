use crate::{
    frontend::parser::{
        error::ParserError, expression::Expression, operators::binary::BinaryOperator,
        statements::Statement,
    },
    BufferedLayer, CharacterIterator, Lexer, Parser,
};

fn parse_statement(command: &str) -> Vec<Result<Statement, ParserError>> {
    let reader = CharacterIterator::new(std::io::Cursor::new(command));
    let lexer = Lexer::new(BufferedLayer::new(reader));
    Parser::new(BufferedLayer::new(lexer)).collect()
}

#[test]
fn select_literal() {
    let command = "select 69;";
    let mut statements = parse_statement(command);

    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");

    let select = Statement::Select {
        select_expressions: vec![Expression::Literal(69.into())],
        from: None,
        predicate: None,
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();

    assert_eq!(parsed, select);
}

#[test]
fn select() {
    let command = "select * from users;";
    let mut statements = parse_statement(command);
    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![Expression::Wildcard],
        from: Some("users".into()),
        predicate: None,
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();

    assert_eq!(parsed, select);
}

#[test]
fn select_with_columns() {
    let command = "select id, name from users;";
    let mut statements = parse_statement(command);
    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: None,
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}

#[test]
fn select_with_predicate() {
    let command = "select id, name from users where id == 1;";
    let mut statements = parse_statement(command);

    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: Some(Expression::Binary {
            left: Box::new(Expression::Identifier("id".into())),
            operator: BinaryOperator::Equals,
            right: Box::new(Expression::Literal(1.into())),
        }),
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}

#[test]
fn select_with_predicate_and_or() {
    let command = "select id, name from users where id == 1 or name == 'Alice';";
    let mut statements = parse_statement(command);

    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: Some(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("id".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal(1.into())),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("name".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal("Alice".into())),
            }),
        }),
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}

#[test]
fn select_with_predicate_and_and() {
    let command = "select id, name from users where id == 1 and name == 'Alice';";
    let mut statements = parse_statement(command);
    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: Some(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("id".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal(1.into())),
            }),
            operator: BinaryOperator::And,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("name".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal("Alice".into())),
            }),
        }),
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}

#[test]
fn select_with_predicate_and_or_and() {
    let command =
        "select id, name from users where id == 1 and (name == 'Alice' or name == 'Bob');";
    let mut statements = parse_statement(command);
    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: Some(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("id".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal(1.into())),
            }),
            operator: BinaryOperator::And,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier("name".into())),
                    operator: BinaryOperator::Equals,
                    right: Box::new(Expression::Literal("Alice".into())),
                }),
                operator: BinaryOperator::Or,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier("name".into())),
                    operator: BinaryOperator::Equals,
                    right: Box::new(Expression::Literal("Bob".into())),
                }),
            }),
        }),
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}

#[test]
fn select_with_predicate_and_or_and_2() {
    let command =
        "select id, name from users where (id == 1 and name == 'Alice') or name == 'Bob';";
    let mut statements = parse_statement(command);
    assert_eq!(statements.len(), 1, "Expected one statement to be parsed");
    let select = Statement::Select {
        select_expressions: vec![
            Expression::Identifier("id".into()),
            Expression::Identifier("name".into()),
        ],
        from: Some("users".into()),
        predicate: Some(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier("id".into())),
                    operator: BinaryOperator::Equals,
                    right: Box::new(Expression::Literal(1.into())),
                }),
                operator: BinaryOperator::And,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier("name".into())),
                    operator: BinaryOperator::Equals,
                    right: Box::new(Expression::Literal("Alice".into())),
                }),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("name".into())),
                operator: BinaryOperator::Equals,
                right: Box::new(Expression::Literal("Bob".into())),
            }),
        }),
    };
    let parsed = statements
        .pop()
        .expect("Expected a statement to be parsed successfully")
        .unwrap();
    assert_eq!(parsed, select);
}
