use std::fmt::Display;

use crate::{error::DBError, parser::statements::Statement};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Analyzer<Statements>
where
    Statements: Iterator<Item = Result<Statement, DBError>>,
{
    statements: Statements,
}

impl<Statements> Analyzer<Statements>
where
    Statements: Iterator<Item = Result<Statement, DBError>>,
{
    pub(crate) fn new(statements: Statements) -> Self {
        Self { statements }
    }

    pub(crate) fn analyze(&mut self, statements: Statement) -> Result<Statement, AnalyzerError> {
        Ok(statements)
    }
}

impl<Statements> Iterator for Analyzer<Statements>
where
    Statements: Iterator<Item = Result<Statement, DBError>>,
{
    type Item = Result<Statement, AnalyzerError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.statements.next()? {
            Ok(statement) => self.analyze(statement),
            Err(err) => Err(AnalyzerError::DBError(err)),
        })
    }
}

#[derive(Debug)]
pub(crate) enum AnalyzerError {
    DBError(DBError),
}

impl Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzerError::DBError(err) => write!(f, "Parser error: {}", err),
        }
    }
}
