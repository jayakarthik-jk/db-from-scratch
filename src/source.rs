use std::io::{BufRead, BufReader, Read};

use crate::common::position::{Position, Span};

// iterates over a source by char. asumes source is valid utf8
pub(crate) struct RawStatement {
    pub(crate) content: String,
    pub(crate) span: Span,
}

impl RawStatement {
    pub(crate) fn iter<'r>(&'r self) -> RawStatementIterator<'r> {
        RawStatementIterator::new(self)
    }

    pub(crate) fn slice(&self, span: &Span) -> &str {
        &self.content
            [span.start.index - self.span.start.index..=span.end.index - self.span.start.index]
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Atom {
    pub(crate) value: char,
    // position in the current raw statement
    pub(crate) position: Position,
}

// Iterator over RawStatement to yield Atoms
pub(crate) struct RawStatementIterator<'r> {
    position: Position,
    raw_statement: &'r RawStatement,
}

impl<'r> RawStatementIterator<'r> {
    pub(crate) fn new(raw_statement: &'r RawStatement) -> Self {
        Self {
            raw_statement,
            position: raw_statement.span.start,
        }
    }
}

impl<'r> Iterator for RawStatementIterator<'r> {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self
            .raw_statement
            .content
            .get(self.position.index..)
            .and_then(|s| s.chars().next());
        ch.map(|c| {
            // update cursor and position
            let atom = Atom {
                value: c,
                position: self.position,
            };

            self.position += c;

            atom
        })
    }
}

// Trait to split a source into raw statements
pub(crate) trait SplitRawStatements<R>
where
    R: Read,
{
    fn split_raw_statements(self) -> SourceIterator<R>;
}

impl<R> SplitRawStatements<R> for R
where
    R: Read,
{
    fn split_raw_statements(self) -> SourceIterator<R> {
        SourceIterator::new(self)
    }
}

/// Responsibility of this struct is to read from the source
/// and yield statements split by semicolon
/// Assumes the source is valid utf8
pub(crate) struct SourceIterator<R>
where
    R: Read,
{
    source: BufReader<R>,
    pub(crate) position: Position,
}

impl<R> SourceIterator<R>
where
    R: Read,
{
    const SEPERATOR: u8 = b';';

    pub(crate) fn new(source: R) -> Self {
        Self {
            source: BufReader::new(source),
            position: Position::default(),
        }
    }
}

impl<R> Iterator for SourceIterator<R>
where
    R: Read,
{
    type Item = RawStatement;

    fn next(&mut self) -> Option<Self::Item> {
        let mut statement = Vec::new();
        let start_position = self.position;
        // take until semicolon
        let read = self
            .source
            .read_until(Self::SEPERATOR, &mut statement)
            .inspect_err(|e| eprintln!("Error reading source: {}", e))
            .ok()?;

        if read == 0 {
            return None;
        }

        let raw_statement = String::from_utf8(statement)
            .expect("Source is not valid utf8")
            .to_string();

        self.position += raw_statement.as_str();

        Some(RawStatement {
            content: raw_statement,
            span: Span {
                start: start_position,
                end: self.position,
            },
        })
    }
}
