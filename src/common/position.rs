use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Position {
    pub(crate) row: usize,
    pub(crate) col: usize,
    // The byte index in the source
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

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
            index: self.index + rhs.index,
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

impl AddAssign<&str> for Position {
    fn add_assign(&mut self, rhs: &str) {
        for ch in rhs.chars() {
            *self += ch;
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
