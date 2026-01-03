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

    pub(crate) fn analyze(&mut self, statements: Statement) -> Result<Statement, DBError> {
        Ok(statements)
    }
}

impl<Statements> Iterator for Analyzer<Statements>
where
    Statements: Iterator<Item = Result<Statement, DBError>>,
{
    type Item = Result<Statement, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.statements
                .next()?
                .and_then(|s| self.analyze(s)),
        )
    }
}
