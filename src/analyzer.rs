use std::fmt::Display;

use crate::{common::layer::Layer, error::DBError, parser::statements::Statement};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Analyzer<ParserLayer>
where
    ParserLayer: Layer<Statement, DBError>,
{
    statements: ParserLayer,
}

impl<ParserLayer> Analyzer<ParserLayer>
where
    ParserLayer: Layer<Statement, DBError>,
{
    pub(crate) fn new(statements: ParserLayer) -> Self {
        Self { statements }
    }

    pub(crate) fn analyze(&mut self, statements: Statement) -> Result<Statement, AnalyzerError> {
        Ok(statements)
    }
}

impl<ParserLayer> Iterator for Analyzer<ParserLayer>
where
    ParserLayer: Layer<Statement, DBError>,
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
