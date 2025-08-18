use std::{
    cell::RefCell,
    fmt::Display,
    io::Read,
    ops::{Add, AddAssign},
};

pub(crate) struct Source<T: Read> {
    reader: RefCell<T>,
    content: RefCell<String>,
}

impl<T: Read> Source<T> {
    pub(crate) fn new(reader: T) -> Self {
        Self {
            reader: RefCell::new(reader),
            content: RefCell::new(String::new()),
        }
    }

    pub(crate) fn iter(&self) -> SourceIterator<T> {
        SourceIterator::new(self)
    }

    fn refill(&self) -> bool {
        let mut content = self.content.borrow_mut();
        let mut temp = [0u8; 1024 * 8]; // 8 KiB buffer
        let mut reader = self.reader.borrow_mut();
        let n = reader.read(&mut temp).expect("Read failed");

        if n == 0 {
            return false;
        }

        content.push_str(std::str::from_utf8(&temp[..n]).expect("Invalid UTF-8 in source"));

        true
    }

    pub(crate) fn char_at(&self, index: usize) -> Option<char> {
        self.content.borrow().chars().nth(index)
    }

    pub(crate) fn slice(&self, span: Span) -> Option<String> {
        let content = self.content.borrow();
        content
            .get(span.start.index..=span.end.index)
            .map(ToString::to_string)
    }

    // pub(crate) fn complete(&self) -> String {
    //     self.content.replace(String::new())
    // }
}
// iterates over a source by char. asumes source is valid utf8

pub(crate) struct SourceIterator<'s, T: Read> {
    source: &'s Source<T>,
    cursor: usize,
    pub(crate) position: Position,
}

impl<'s, T: Read> SourceIterator<'s, T> {
    pub(crate) fn new(source: &'s Source<T>) -> Self {
        Self {
            source,
            cursor: 0,
            position: Position::default(),
        }
    }
}

impl<'a, T: Read> Iterator for SourceIterator<'a, T> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        let len = {
            let content = self.source.content.borrow();
            content.len()
        };

        if self.cursor >= len && !self.source.refill() {
            return None;
        }

        // use char_at to get the next Character
        self.source.char_at(self.cursor).map(|c| {
            // update cursor and position
            let character = Character {
                value: c,
                position: self.position,
            };

            self.cursor += c.len_utf8();
            self.position += c;

            character
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Character {
    pub(crate) value: char,
    pub(crate) position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Position {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) index: usize,
}

impl Add<usize> for Position {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            row: self.row,
            col: self.col + rhs,
            index: self.index + rhs,
        }
    }
}

impl AddAssign<char> for Position {
    fn add_assign(&mut self, rhs: char) {
        let len = rhs.len_utf8();
        self.index += len;
        if rhs == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += len;
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({}, {})", self.start, self.end)
    }
}
