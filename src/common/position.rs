use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Position {
    // The byte index in the current working statement.
    // only for internal use, not for display
    pub(crate) index: usize,
    pub(crate) absolute: AbsolutePosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct AbsolutePosition {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Add<usize> for Position {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            absolute: AbsolutePosition {
                row: self.absolute.row,
                col: self.absolute.col + rhs,
            },
            index: self.index + rhs,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            absolute: AbsolutePosition {
                row: self.absolute.row + rhs.absolute.row,
                col: self.absolute.col + rhs.absolute.col,
            },
            index: self.index + rhs.index,
        }
    }
}

impl AddAssign<char> for Position {
    fn add_assign(&mut self, rhs: char) {
        let len = rhs.len_utf8();
        self.index += len;
        if rhs == '\n' {
            self.absolute.row += 1;
            self.absolute.col = 0;
        } else {
            self.absolute.col += len;
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
        write!(f, "{}:{}", self.absolute.row, self.absolute.col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start: Position,
    pub(crate) end: Position,
}
