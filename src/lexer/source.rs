use std::{cell::RefCell, io::Read};

use crate::common::position::Position;

pub(crate) struct Source<R: Read> {
    reader: RefCell<R>,
    content: RefCell<String>,
}

impl<R: Read> Source<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader: RefCell::new(reader),
            content: RefCell::new(String::new()),
        }
    }

    pub(crate) fn iter(&self) -> SourceIterator<R> {
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

    // pub(crate) fn slice(&self, span: Span) -> Option<String> {
    //     let content = self.content.borrow();
    //     content
    //         .get(span.start.index..=span.end.index)
    //         .map(ToString::to_string)
    // }

    // pub(crate) fn complete(&self) -> String {
    //     self.content.replace(String::new())
    // }
}
// iterates over a source by char. asumes source is valid utf8

pub(crate) struct SourceIterator<'s, R: Read> {
    source: &'s Source<R>,
    cursor: usize,
    pub(crate) position: Position,
}

impl<'s, R: Read> SourceIterator<'s, R> {
    pub(crate) fn new(source: &'s Source<R>) -> Self {
        Self {
            source,
            cursor: 0,
            position: Position::default(),
        }
    }
}

impl<'a, R: Read> Iterator for SourceIterator<'a, R> {
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
